mod jcloud_urlgroup;
mod url_response;
mod args;

use std::process::Command;
use rocket::{routes, get, launch, State};
use crate::jcloud_urlgroup::JCloudURLGroup;
use regex::Regex;
use lazy_static::lazy_static;
use rocket::serde::json::Json;
use crate::args::Args;
use crate::url_response::URLResponse;
use clap::Parser;

fn get_urls() -> Vec<String> {
    lazy_static! {
        static ref RE_URLS: Regex = Regex::new(r"\s([a-z]+?://.*jina\.ai)").unwrap();
    }
    let jc_output = Command::new("jcloud")
        .args(["list"])
        .output()
        .expect("failed to execute process");
    let urls = RE_URLS.captures_iter(&String::from_utf8_lossy(&jc_output.stdout)).map(|g| {
        g[1].to_string()
    }).collect::<Vec<String>>();
    return urls;
}

fn check_jcloud(url_group: &State<JCloudURLGroup>) {
    let urls = get_urls();
    let my_url = url_group.my_url.read().unwrap();
    if my_url.is_none() || !urls.contains(my_url.as_ref().unwrap()) {
        // We need to start the new jcloud instance
    }
}

#[get("/")]
fn index(url_group: &State<JCloudURLGroup>) -> Json<URLResponse> {
    check_jcloud(url_group);

    return Json(URLResponse {
        endpoint: "".to_string()
    });
}

#[launch]
fn rocket() -> _ {
    let args = Args::parse();
    rocket::build().manage(JCloudURLGroup::new(args)).mount("/", routes![index])
}
