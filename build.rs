use std::fs;
use std::process::Command;

use regex::Regex;

use walkdir::WalkDir;

fn main() {
    replace_version_in_rs(
        "true"
            == std::env::var("UPDATE_ALL_FILES").unwrap_or(String::from("false").to_lowercase()),
    );
}

fn replace_version_in_rs(update_all_files: bool) {
    let latest_version = format!(
        "{}.{}",
        get_latest_git_commit_hash(true),
        get_latest_git_commit_time()
    );

    // Replace version string in .rs files
    let version_regex =
        Regex::new(r#"pub static APP_VERSION: &str = "([0-9a-f]{7})\.(\d{8})\.(\d{6})";"#).unwrap();
    let version_replacement = format!(r#"pub static APP_VERSION: &str = "{}";"#, latest_version);
    let files = if update_all_files {
        find_rs_files()
    } else {
        let file = String::from("./src/version.rs");
        if !std::path::Path::new(&file).exists() {
            let mut text = version_replacement.clone();
            text += "\n";
            fs::write(&file, text).expect("fs::write error");
        }
        vec![file]
    };
    for file in files {
        println!("file: {}", &file);
        let original_content = fs::read_to_string(&file).expect("Failed to read file");
        let replaced_content = version_regex.replace_all(&original_content, &version_replacement);
        if original_content != replaced_content {
            println!("fs::write, file: {}, version: {}", &file, &latest_version);
            fs::write(&file, replaced_content.as_ref()).expect("fs::write error");
        }
    }
}

fn get_latest_git_commit_hash(short: bool) -> String {
    // Run Git command to get the latest commit hash
    let output = Command::new("git")
        .args(&[
            "log",
            "-1",
            if short {
                "--pretty=format:%h"
            } else {
                "--pretty=format:%H"
            },
        ])
        .output()
        .expect("Command::new(git log) error");

    return String::from_utf8_lossy(&output.stdout).trim().to_string();
}

fn get_latest_git_commit_time() -> String {
    // Run Git command to get the latest commit hash
    let output = Command::new("git")
        .args(&["log", "-1", "--format=%ad", "--date=format:%Y%m%d.%H%M%S"])
        .output()
        .expect("Command::new(git log) error");

    return String::from_utf8_lossy(&output.stdout).trim().to_string();
}

fn find_rs_files() -> Vec<String> {
    let mut files = Vec::new();
    let walker = WalkDir::new("./src")
        .into_iter()
        .filter_entry(|e| !is_hidden(e)) // skip hidden files
        .filter_map(|e| e.ok());
    for entry in walker {
        let path = entry.path();
        if let Some(ext) = path.extension() {
            if ext == "rs" {
                if let Some(file_name) = path.to_str() {
                    // skip build.rs
                    if !file_name.ends_with("build.rs") {
                        files.push(file_name.to_owned());
                    }
                }
            }
        }
    }
    files
}

fn is_hidden(entry: &walkdir::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}
