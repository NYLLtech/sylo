use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, Write},
    path::PathBuf,
};

use crate::{
    DIRECTORY_NAME, IGNORE_FILE_NAME, MANIFEST_FILE_NAME, manifest::Manifest, utils::write_manifest,
};

pub fn create_ignore_file(path: &PathBuf) {
    let ignore_file: PathBuf = path.join(IGNORE_FILE_NAME);
    let result: Result<File, io::Error> = File::create_new(&ignore_file);

    if result.is_err() {
        eprintln!(
            "Could not create ignore file ({IGNORE_FILE_NAME}): {}",
            result.unwrap_err()
        );
    };
}

pub fn convert_rules(
    tags: &mut HashMap<String, String>,
    rules: Vec<(String, String)>,
) -> HashMap<String, Vec<String>> {
    let mut converted: HashMap<String, Vec<String>> = HashMap::new();

    for (rule, rule_tags) in rules {
        let rule_tags: Vec<String> = rule_tags
            .split(',')
            .map(str::trim)
            .filter(|tag| !tag.is_empty())
            .map(|tag| {
                tags.entry(tag.to_string())
                    .or_insert_with(|| tag.to_string());

                tag.to_string()
            })
            .collect();

        converted.insert(rule, rule_tags);
    }

    converted
}

pub fn parse_options(
    patterns: Vec<(String, String)>,
    classifiers: Vec<(String, String)>,
) -> (
    HashMap<String, String>,
    HashMap<String, Vec<String>>,
    HashMap<String, Vec<String>>,
) {
    let mut tags: HashMap<String, String> = HashMap::new();
    let converted_patterns: HashMap<String, Vec<String>> = convert_rules(&mut tags, patterns);
    let converted_classifiers: HashMap<String, Vec<String>> = convert_rules(&mut tags, classifiers);

    (tags, converted_patterns, converted_classifiers)
}

pub fn initialize_manifest(
    sylo_path: PathBuf,
    patterns: Vec<(String, String)>,
    classifiers: Vec<(String, String)>,
) {
    let manifest_path: PathBuf = sylo_path.join(MANIFEST_FILE_NAME);

    if let Err(error) = File::create_new(&manifest_path) {
        eprintln!("Could not create manifest ({MANIFEST_FILE_NAME}): {error}",);

        return;
    };

    let (tags, patterns, classifiers) = parse_options(patterns, classifiers);
    let manifest: Manifest = Manifest::latest(tags, patterns, classifiers);

    write_manifest(&manifest_path, manifest);
}

pub fn init(
    path: PathBuf,
    ignore: bool,
    patterns: Vec<(String, String)>,
    classifiers: Vec<(String, String)>,
) {
    println!("Initializing workspace ({})...", path.display());

    if ignore {
        create_ignore_file(&path);
    }

    let sylo_path: PathBuf = path.join(DIRECTORY_NAME);
    let result: Result<(), io::Error> = fs::create_dir(&sylo_path);

    if result.is_err() {
        eprintln!(
            "Could not create Sylo directory ({DIRECTORY_NAME}): {}",
            result.unwrap_err(),
        );
    }

    initialize_manifest(sylo_path, patterns, classifiers);
}
