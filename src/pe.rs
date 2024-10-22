use anyhow::Result;
use object::Object;
use std::{collections::HashSet, fs, path::PathBuf};

/// Target binary to be analyzed.
#[derive(Debug)]
pub struct TargetBinary {
    /// Path to the binary.
    pub path: PathBuf,

    /// Imported symbols (i.e. DLLs).
    pub imports: Vec<String>,

    /// Exported symbols (i.e. functions).
    pub exports: Vec<String>,
}

impl TargetBinary {
    /// Build a new target binary from a filesystem path.
    pub fn from(path: &PathBuf) -> Result<Self> {
        // Read binary from disk and parse it.
        let binary = fs::read(&path)?;
        let binary = object::File::parse(&*binary)?;

        // HashSet to store results by discarding duplicates.
        let mut set = HashSet::new();

        // Loop over the imports section and build a list of imported symbols.
        for entry in binary.imports()? {
            let symbol = String::from_utf8_lossy(entry.library());
            set.insert(symbol);
        }

        // Sort list of imported symbols.
        let mut imports = set.drain().map(|s| s.into_owned()).collect::<Vec<String>>();
        imports.sort();

        // Loop over the exports section and build a list of exported symbols.
        for entry in binary.exports()? {
            let symbol = String::from_utf8_lossy(entry.name());
            set.insert(symbol);
        }

        // Sort list of exported symbols.
        let mut exports = set.drain().map(|s| s.into_owned()).collect::<Vec<String>>();
        exports.sort();

        Ok(Self {
            path: path.clone(),
            imports,
            exports,
        })
    }
}
