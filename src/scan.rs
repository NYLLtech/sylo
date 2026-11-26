use std::{
    collections::HashSet,
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
};

use globset::{Glob, GlobSet, GlobSetBuilder};
use walkdir::{DirEntry, WalkDir};

use crate::{
    DIRECTORY_NAME, IGNORE_FILE_NAME, MANIFEST_FILE_NAME,
    classifier::Matcher,
    manifest::Manifest,
    utils::{load_manifest, normalize_path, write_manifest},
};

#[derive(Debug)]
pub struct ScanResult {
    pub new_files: Vec<PathBuf>,
    pub missing_files: Vec<PathBuf>,
}

fn load_ignore_patterns(path: &Path) -> io::Result<GlobSet> {
    let ignore_path: PathBuf = path.join(IGNORE_FILE_NAME);
    let mut builder: GlobSetBuilder = GlobSetBuilder::new();

    if !ignore_path.exists() {
        return builder.build().map_err(io::Error::other);
    }

    let mut content: String = fs::read_to_string(ignore_path)?;

    content.push_str("\n.sylo/*");

    for line in content.lines() {
        let line: &str = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let glob: Glob =
            Glob::new(line).map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;

        builder.add(glob);
    }

    builder.build().map_err(io::Error::other)
}

fn do_scan(path: &PathBuf, manifest: &Manifest) -> io::Result<ScanResult> {
    let ignore: GlobSet = load_ignore_patterns(&path)?;

    let mut discovered: HashSet<String> = HashSet::new();
    let mut new_files: Vec<PathBuf> = Vec::new();

    for entry in WalkDir::new(&path).follow_links(false) {
        let entry: DirEntry = entry.map_err(io::Error::other)?;
        let entry_path: &Path = entry.path();

        let relative_entry_path: &Path = entry_path.strip_prefix(&path).unwrap();
        let relative_entry_path_string: String =
            relative_entry_path.to_string_lossy().replace('\\', "/");

        if ignore.is_match(&relative_entry_path_string) {
            continue;
        }

        if !entry.file_type().is_file() {
            continue;
        }

        discovered.insert(relative_entry_path_string.clone());

        if !manifest.files.contains_key(&relative_entry_path_string) {
            new_files.push(relative_entry_path.to_path_buf());
        }
    }

    let missing_files: Vec<PathBuf> = manifest
        .files
        .keys()
        .filter(|path| !discovered.contains(*path))
        .map(PathBuf::from)
        .collect();

    Ok(ScanResult {
        new_files,
        missing_files,
    })
}

fn display_files(items: &[PathBuf]) -> String {
    if items.is_empty() {
        return String::from("- (none)");
    }

    items
        .iter()
        .map(|item| format!("- {}", item.display()))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn scan(path: PathBuf) {
    let sylo_directory: PathBuf = path.join(DIRECTORY_NAME);
    let manifest_path: PathBuf = sylo_directory.join(MANIFEST_FILE_NAME);

    let mut manifest: Manifest = load_manifest(&manifest_path);
    let matcher: Matcher = Matcher::new(&manifest);

    let scan_result: ScanResult = match do_scan(&path, &manifest) {
        Ok(scan_result) => scan_result,
        Err(error) => {
            eprintln!("Could not scan the workspace: {}", error);

            return;
        }
    };

    println!(
        "Scan results:\n\nMissing files ({}):\n{}\n\nNew files ({}):\n{}\n",
        scan_result.missing_files.len(),
        display_files(&scan_result.missing_files),
        scan_result.new_files.len(),
        display_files(&scan_result.new_files),
    );

    if scan_result.missing_files.is_empty() && scan_result.new_files.is_empty() {
        println!("Nothing to do.");

        return;
    }

    let mut input: String = String::new();

    print!("Update manifest? (Y/n): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();

    if input.trim().to_lowercase() == "n" {
        return;
    };

    println!("Updating manifest...");

    for new_file in scan_result.new_files {
        manifest
            .files
            .insert(normalize_path(&new_file), matcher.get_tags(new_file));
    }

    for missing_file in scan_result.missing_files {
        manifest.files.remove(&normalize_path(&missing_file));
    }

    write_manifest(&manifest_path, manifest);
}
