use std::{fs, io::Write, path::PathBuf};

use crate::{MANIFEST_FILE_NAME, manifest::Manifest};

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

    let content = match toml::to_string(&manifest) {
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

pub fn normalize_path(file: &PathBuf) -> String {
    file.to_string_lossy().to_string().replace('\\', "/")
}
