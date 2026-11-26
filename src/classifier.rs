use std::{collections::BTreeMap, path::PathBuf};

use globset::{Glob, GlobMatcher};

use crate::{manifest::Manifest, utils::normalize_path};

pub struct Matcher {
    pub classifiers: Vec<(GlobMatcher, Vec<String>)>,
    pub patterns: Vec<(GlobMatcher, Vec<String>)>,
}

fn compile_rules(rules: &BTreeMap<String, Vec<String>>) -> Vec<(GlobMatcher, Vec<String>)> {
    let mut compiled_rules: Vec<(GlobMatcher, Vec<String>)> = Vec::with_capacity(rules.len());

    for (rule, tags) in rules {
        let glob: Glob = match Glob::new(&rule) {
            Ok(glob) => glob,
            Err(error) => {
                eprintln!("Could not parse {} as a glob pattern: {}", rule, error);

                continue;
            }
        };

        compiled_rules.push((glob.compile_matcher(), tags.clone()));
    }

    compiled_rules
}

impl Matcher {
    pub fn new(manifest: &Manifest) -> Self {
        let classifiers: Vec<(GlobMatcher, Vec<String>)> = compile_rules(&manifest.classifiers);

        let patterns: Vec<(GlobMatcher, Vec<String>)> = compile_rules(&manifest.patterns);

        Matcher {
            classifiers,
            patterns,
        }
    }

    pub fn get_tags(&self, path: PathBuf) -> Vec<String> {
        let mut tags: Vec<String> = Vec::new();
        let path: String = normalize_path(&path);

        for (classifier, tags_classifier) in &self.classifiers {
            if !classifier.is_match(&path) {
                continue;
            }

            tags.append(&mut tags_classifier.clone());
        }

        tags
    }
}
