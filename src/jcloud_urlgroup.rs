use std::sync::RwLock;

pub struct JCloudURLGroup {
    pub my_url: RwLock<Option<String>>
}

impl JCloudURLGroup {
    pub fn new() -> Self {
        return Self {
            my_url: RwLock::new(None)
        };
    }
}