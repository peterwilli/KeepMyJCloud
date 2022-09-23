use std::sync::RwLock;
use std::time::Instant;

use url::Url;

pub struct State {
    pub my_url: RwLock<Option<Url>>,
    pub last_checked: RwLock<Option<Instant>>
}

impl State {
    pub fn new() -> Self {
        return Self {
            last_checked: RwLock::new(None),
            my_url: RwLock::new(None)
        };
    }
}