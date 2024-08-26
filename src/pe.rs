use anyhow::Result;
use object::Object;
use std::{collections::HashSet, fs, path::PathBuf};

/// A target binary to be analyzed.
#[derive(Debug)]
pub struct TargetBinary {
    /// Path to the binary.
    pub path: PathBuf,

    /// Imported DLLs.
    pub dlls: Vec<String>,
}

impl TargetBinary {
    /// Build a new target binary from a filesystem path.
    pub fn from(path: &PathBuf) -> Result<Self> {
        // Read binary from disk and parse it.
        let binary = fs::read(&path)?;
        let binary = object::File::parse(&*binary)?;

        // Loop over the imports section and build a list of imported DLLs
        // discarding duplicates.
        let mut set = HashSet::new();
        for entry in binary.imports()? {
            let dll = String::from_utf8_lossy(entry.library());
            set.insert(dll);
        }

        // Build list of imported DLLs from HashSet.
        let mut dlls = set
            .into_iter()
            .map(|dll| dll.into_owned())
            .collect::<Vec<String>>();
        dlls.sort();

        Ok(Self {
            path: path.clone(),
            dlls,
        })
    }

    /// Print contents to stdout.
    pub fn print(&self) {
        // Write path to target binary.
        print!("{}: [", self.path.display());

        // If dlls is empty, print a a single space between square brackets.
        if self.dlls.is_empty() {
            println!("\n    NO IMPORTS FOUND!\n]");
        } else {
            println!();
            // Write each DLL entry with 4 spaces as indentation.
            for dll in &self.dlls {
                println!("    {dll}");
            }
            println!("]");
        }
    }
}
