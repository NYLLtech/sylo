use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const MANIFEST_VERSION: u32 = 0;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub version: u32,

    #[serde(default)]
    pub tags: HashMap<String, String>,

    #[serde(default)]
    pub patterns: HashMap<String, Vec<String>>,

    #[serde(default)]
    pub classifiers: HashMap<String, Vec<String>>,

    #[serde(default)]
    pub files: HashMap<String, Vec<String>>,
}

impl Manifest {
    pub fn new() -> Self {
        Self::latest(HashMap::new(), HashMap::new(), HashMap::new())
    }

    pub fn latest(
        tags: HashMap<String, String>,
        patterns: HashMap<String, Vec<String>>,
        classifiers: HashMap<String, Vec<String>>,
    ) -> Self {
        Manifest {
            version: MANIFEST_VERSION,
            tags,
            patterns,
            classifiers,
            files: HashMap::new(),
        }
    }
}
