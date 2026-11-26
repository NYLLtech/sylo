use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

const INDEX_VERSION: u32 = 0;

#[derive(Serialize, Deserialize)]
pub struct Index {
    pub version: u32,
    pub files: BTreeMap<String, Vec<String>>,
}

impl Index {
    pub fn new() -> Self {
        Index {
            version: INDEX_VERSION,
            files: BTreeMap::new(),
        }
    }
}
