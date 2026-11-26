use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

const MANIFEST_VERSION: u32 = 0;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub version: u32,

    #[serde(default)]
    pub tags: BTreeMap<String, String>,

    #[serde(default)]
    pub patterns: BTreeMap<String, Vec<String>>,

    #[serde(default)]
    pub classifiers: BTreeMap<String, Vec<String>>,

    #[serde(default)]
    pub files: BTreeMap<String, Vec<String>>,
}

impl Manifest {
    pub fn new() -> Self {
        Self::latest(BTreeMap::new(), BTreeMap::new(), BTreeMap::new())
    }

    pub fn latest(
        tags: BTreeMap<String, String>,
        patterns: BTreeMap<String, Vec<String>>,
        classifiers: BTreeMap<String, Vec<String>>,
    ) -> Self {
        Manifest {
            version: MANIFEST_VERSION,
            tags,
            patterns,
            classifiers,
            files: BTreeMap::new(),
        }
    }
}
