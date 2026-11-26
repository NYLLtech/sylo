use std::{fs, io::Write, path::PathBuf};

use crate::{INDEX_FILE_NAME, MANIFEST_FILE_NAME, index::Index, manifest::Manifest};

pub fn load_manifest(manifest_path: &PathBuf) -> Manifest {
    if !manifest_path.exists() {
        return Manifest::new();
    }

    let content: String = match fs::read_to_string(manifest_path) {
        Ok(content) => content,
        Err(_) => return Manifest::new(),
    };

    toml::from_str(&content).unwrap_or(Manifest::new())
}

pub fn write_manifest(manifest_path: &PathBuf, manifest: Manifest) {
    let mut file: fs::File = match fs::File::create(manifest_path) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Could not create/open manifest ({MANIFEST_FILE_NAME}): {error}");

            return;
        }
    };

    let content: String = match toml::to_string(&manifest) {
        Ok(content) => content,
        Err(error) => {
            eprintln!("Could not serialize manifest ({MANIFEST_FILE_NAME}): {error}",);

            return;
        }
    };

    if let Err(error) = file.write_all(content.as_bytes()) {
        eprintln!("Could not write manifest ({MANIFEST_FILE_NAME}): {error}",);
    }
}

pub fn load_index(index_path: &PathBuf) -> Index {
    if !index_path.exists() {
        return Index::new();
    }

    let content: String = match fs::read_to_string(index_path) {
        Ok(content) => content,
        Err(_) => return Index::new(),
    };

    serde_json::from_str(&content).unwrap_or(Index::new())
}

pub fn write_index(index_path: &PathBuf, index: Index) {
    let mut file: fs::File = match fs::File::create(index_path) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Could not create/open index ({INDEX_FILE_NAME}): {error}");

            return;
        }
    };

    let content: String = match serde_json::to_string_pretty(&index) {
        Ok(content) => content,
        Err(error) => {
            eprintln!("Could not serialize index ({INDEX_FILE_NAME}): {error}",);

            return;
        }
    };

    if let Err(error) = file.write_all(content.as_bytes()) {
        eprintln!("Could not write index ({INDEX_FILE_NAME}): {error}",);
    }
}

pub fn normalize_path(file: &PathBuf) -> String {
    file.to_string_lossy().to_string().replace('\\', "/")
}
