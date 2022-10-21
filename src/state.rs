use std::sync::RwLock;
use std::time::Instant;
use url::Url;

pub struct State {
    pub project_name: String,
    pub my_url: RwLock<Option<Url>>,
    pub last_checked: RwLock<Option<Instant>>
}

impl State {
    pub fn new(project_name: String) -> Self {
        return Self {
            last_checked: RwLock::new(None),
            my_url: RwLock::new(None),
            project_name
        };
    }
}