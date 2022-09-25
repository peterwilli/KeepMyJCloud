use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use clap::Parser;
use lazy_static::lazy_static;
use log::{debug, error};
use regex::Regex;
use rocket::{get, launch, routes, State as RocketState};
use rocket::serde::json::Json;
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

fn start_instance(flow_yml_path: &str) -> Result<Url, &'static str> {
    let jc_output = run_jcloud(&["deploy", flow_yml_path]);
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
    let my_url = state.my_url.read().unwrap();
    if my_url.is_none() || !urls.contains(my_url.as_ref().unwrap()) {
        drop(my_url);
        let url = start_instance(flow_yml_path).unwrap();
        *state.my_url.write().unwrap() = Some(url);
    }
}

#[get("/")]
fn index(state: &RocketState<State>, args: &RocketState<Args>) -> Json<URLResponse> {
    let last_checked_read_lock = state.last_checked.read().unwrap();
    let should_check = if last_checked_read_lock.is_some() {
        last_checked_read_lock.unwrap().elapsed() > Duration::from_secs(10)
    } else {
        true
    };
    drop(last_checked_read_lock);
    if should_check {
        check_jcloud(state, &args.flow_yml_path);
        *state.last_checked.write().unwrap() = Some(Instant::now());
    }
    let mut url = state.my_url.read().unwrap().as_ref().unwrap().to_string();
    // Rust tends to set a trailing slash at the end of a URL which may or may not work with Jina.
    // I choose to remove it to follow their conventions more closely.
    if url.ends_with("/") {
        url.pop();
    }
    return Json(URLResponse {
        endpoint: url
    });
}

#[get("/info")]
fn info() -> Json<InfoResponse> {
    return Json(InfoResponse::new());
}

#[launch]
fn rocket() -> _ {
    env_logger::init();
    let args = Args::parse();
    let state = State::new();
    if args.current_jcloud_url.is_some() {
        *state.my_url.write().unwrap() = Some(Url::parse(args.current_jcloud_url.as_ref().unwrap()).unwrap());
    }
    rocket::build().manage(args).manage(state).mount("/", routes![index, info])
}
