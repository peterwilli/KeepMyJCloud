use std::process::Command;

use clap::Parser;
use lazy_static::lazy_static;
use log::debug;
use regex::Regex;
use rocket::{get, launch, routes, State};
use rocket::serde::json::Json;
use url::{ParseError, Url};

use crate::args::Args;
use crate::jcloud_urlgroup::JCloudURLGroup;
use crate::url_response::URLResponse;

mod jcloud_urlgroup;
mod url_response;
mod args;

lazy_static! {
    static ref RE_URLS: Regex = Regex::new(r"\s([a-z]+?://.*jina\.ai)").unwrap();
}

fn run_jcloud(args: &[&str]) -> String {
    debug!("Calling jcloud with arguments: {:?}", args);
    let jc_output = Command::new("jcloud")
        .args(args)
        .output()
        .expect("failed to execute process");
    let output_str = String::from_utf8_lossy(&jc_output.stdout).to_string();
    debug!("jcloud {:?} output: {}", args, output_str);
    return output_str;
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

fn check_jcloud(state: &State<JCloudURLGroup>, flow_yml_path: &str) {
    let urls = get_urls();
    let my_url = state.my_url.read().unwrap();
    if my_url.is_none() || !urls.contains(my_url.as_ref().unwrap()) {
        let url = start_instance(flow_yml_path).unwrap();
        *state.my_url.write().unwrap() = Some(url);
    }
}

#[get("/")]
fn index(state: &State<JCloudURLGroup>, args: &State<Args>) -> Json<URLResponse> {
    check_jcloud(state, &args.flow_yml_path);
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

#[launch]
fn rocket() -> _ {
    env_logger::init();
    let args = Args::parse();
    let state = JCloudURLGroup::new();
    if args.current_jcloud_url.is_some() {
        *state.my_url.write().unwrap() = Some(Url::parse(args.current_jcloud_url.as_ref().unwrap()).unwrap());
    }
    rocket::build().manage(args).manage(state).mount("/", routes![index])
}
