use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use clap::Parser;
use lazy_static::lazy_static;
use log::{debug, error};
use regex::Regex;
use rocket::{Config, get, launch, routes, State as RocketState};
use rocket::serde::json::Json;
use rocket_cache_response::CacheResponse;
use url::Url;

use crate::args::Args;
use crate::info_response::InfoResponse;
use crate::state::State;
use crate::url_response::URLResponse;

mod state;
mod url_response;
mod args;
mod info_response;

lazy_static! {
    static ref RE_URLS: Regex = Regex::new(r"\s([a-z]+?://.*jina\.ai)").unwrap();
    static ref RE_URL_NAME: Regex = Regex::new(r"/([a-z]*?)-").unwrap();
}

fn run_jcloud(args: &[&str]) -> String {
    debug!("Calling jcloud with arguments: {:?}", args);
    let mut child = Command::new("jcloud")
        .args(args)
        .stdout(Stdio::piped())
        .spawn().expect("Spawn failed");

    let stdout = child.stdout.take().unwrap();
    let mut reader = BufReader::new(stdout);
    let mut lines = String::new();
    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(_) => {
                debug!("Line: {}", line);
                if line.is_empty() {
                    debug!("Line is empty, time to close off!");
                    break;
                }
                if line.to_lowercase().find("survey").is_some() {
                    debug!("Command is asking for something, killing!");
                    child.kill().expect("Kill failed");
                    break;
                }
                lines.push_str(&line);
            }
            Err(e) => {
                error!("Error: {:?}", e);
            }
        }
    }
    return lines;
}

fn get_urls() -> Vec<Url> {
    let jc_output = run_jcloud(&["list"]);
    let urls = RE_URLS.captures_iter(&jc_output).map(|g| {
        Url::parse(&g[1]).unwrap()
    }).collect::<Vec<Url>>();
    return urls;
}

fn start_instance(flow_yml_path: &str, project_name: &str) -> Result<Url, &'static str> {
    let jc_output = run_jcloud(&["deploy", format!("--name={}", project_name).as_ref(), flow_yml_path]);
    let captures = match RE_URLS.captures(&jc_output) {
        Some(captures) => {
            captures
        }
        None => {
            return Err("No url found in deployment");
        }
    };
    return Ok(Url::parse(&captures[1]).unwrap());
}

fn check_jcloud(state: &RocketState<State>, flow_yml_path: &str) {
    let urls = get_urls();
    let mut current_url = None;
    for url in urls {
        match RE_URL_NAME.captures(url.as_str()) {
            Some(captures) => {
                let name = &captures[1];
                if name == state.project_name {
                    current_url = Some(url);
                    break;
                }
            }
            None => {

            }
        }
    }

    if current_url.is_none() {
        match start_instance(flow_yml_path, &state.project_name) {
            Ok(url) => {
                *state.my_url.write().unwrap() = Some(url);
            }
            Err(_) => {

            }
        };
    }
    else {
        *state.my_url.write().unwrap() = current_url;
    }
}

#[get("/")]
fn index(state: &RocketState<State>, args: &RocketState<Args>) -> CacheResponse<Json<URLResponse>> {
    let last_checked_read_lock = state.last_checked.read().unwrap();
    let should_check = args.flow_yml_path.is_some() && if last_checked_read_lock.is_some() {
        last_checked_read_lock.unwrap().elapsed() > Duration::from_secs(args.check_delay as u64)
    } else {
        true
    };
    drop(last_checked_read_lock);
    if should_check {
        check_jcloud(state, args.flow_yml_path.as_ref().unwrap());
        *state.last_checked.write().unwrap() = Some(Instant::now());
    }
    let url_lock = state.my_url.read().unwrap();
    let mut url = if url_lock.is_some() {
        url_lock.as_ref().unwrap().to_string()
    }
    else {
        // Use the alternate URL if exists
        args.alternate_url.as_ref().expect("No alternate URL while Jina is not available!").to_string()
    };
    // Rust tends to set a trailing slash at the end of a URL which may or may not work with Jina.
    // I choose to remove it to follow their conventions more closely.
    if url.ends_with("/") {
        url.pop();
    }
    return CacheResponse::Public {
        responder: Json(URLResponse {
            endpoint: url
        }),
        max_age: args.check_delay as u32,
        must_revalidate: false
    }
}

#[get("/info")]
fn info() -> CacheResponse<Json<InfoResponse>> {
    return CacheResponse::Public {
        responder: Json(InfoResponse::new()),
        max_age: 10,
        must_revalidate: false
    };
}

#[launch]
fn rocket() -> _ {
    env_logger::init();
    let args = Args::parse();
    if !args.project_name.chars().all(char::is_alphanumeric) {
        panic!("Project name must be only alphanumeric characters!");
    }
    let config = Config {
        address: args.host,
        port: args.port,
        ..Config::debug_default()
    };
    let state = State::new(args.project_name.clone());
    rocket::custom(&config).manage(args).manage(state).mount("/", routes![index, info])
}
