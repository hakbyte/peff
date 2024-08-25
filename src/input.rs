use std::{
    collections::HashSet,
    path::{self, PathBuf},
};
use walkdir::WalkDir;

/// Build a file list from command line args. It recursively searches for files
/// within directories and removes duplicates.
pub fn build_list(input: Vec<String>) -> Vec<PathBuf> {
    let mut set = HashSet::new();
    for i in input {
        let path = path::PathBuf::from(i);
        if path.is_file() {
            set.insert(path);
        } else if path.is_dir() {
            // If any input path is a directory, recursively walks the directory
            // to retrieve its files. Silently ignore permission errors.
            for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
                if entry.file_type().is_file() {
                    let path = path::PathBuf::from(entry.path());
                    set.insert(path);
                }
            }
        }
    }

    // Build final list from HashSet.
    set.into_iter()
        .map(|path| path.to_owned())
        .collect::<Vec<PathBuf>>()
}
