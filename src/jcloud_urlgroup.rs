use std::sync::RwLock;

use url::Url;

pub struct JCloudURLGroup {
    pub my_url: RwLock<Option<Url>>
}

impl JCloudURLGroup {
    pub fn new() -> Self {
        return Self {
            my_url: RwLock::new(None)
        };
    }
}