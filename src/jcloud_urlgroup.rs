use std::sync::RwLock;
use crate::Args;

pub struct JCloudURLGroup {
    pub args: Args,
    pub my_url: RwLock<Option<String>>
}

impl JCloudURLGroup {
    pub fn new(args: Args) -> Self {
        return Self {
            args,
            my_url: RwLock::new(None)
        };
    }
}