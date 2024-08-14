
use std::fs;
use std::process::Command;

use regex::Regex;

use walkdir::WalkDir;



fn main() {
    replace_version_in_rs();
}


fn replace_version_in_rs() {
    let latest_version = format!("{}.{}", get_latest_git_commit_hash(true), get_latest_git_commit_time());

    // Replace version string in .rs files
    let version_regex = Regex::new(r#"let\s+app_version\s*=\s*.*""#).unwrap();
    let version_replacement = format!(r#"let app_version = "{}""#, latest_version);
    let files = find_rs_files();
    for file in files {
        let original_content = fs::read_to_string(&file).expect("Failed to read file");
        let replaced_content = version_regex.replace_all(&original_content, &version_replacement);
        if original_content != replaced_content {
            println!("fs::write, file: {}, version: {}", &file, &latest_version);
            fs::write(&file, replaced_content.as_ref()).expect("Failed to write file");
        }
    }
}



fn get_latest_git_commit_hash(short: bool) -> String {
    // Run Git command to get the latest commit hash
    let output = Command::new("git")
        .args(&[
            "log",
            "-1",
            if short {"--pretty=format:%h"} else {"--pretty=format:%H"}
        ])
        .output()
        .expect("Failed to execute git log command");

    return String::from_utf8_lossy(&output.stdout).trim().to_string();
}


fn get_latest_git_commit_time() -> String {
    // Run Git command to get the latest commit hash
    let output = Command::new("git")
        .args(&[
            "log",
            "-1",
            "--format=%ad",
            "--date=format:%Y%m%d.%H%M%S"
        ])
        .output()
        .expect("Failed to execute git log command");

    return String::from_utf8_lossy(&output.stdout).trim().to_string();
}


fn find_rs_files() -> Vec<String> {
    let mut files = Vec::new();
    let walker = WalkDir::new("./src")
        .into_iter()
        .filter_entry(|e| !is_hidden(e))  // skip hidden files
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
