use std::fs;
use std::path::PathBuf;

use anyhow::Result;

/// Lists a directory files without recursing
pub fn get_files_from_dir<F>(path: PathBuf, f: F) -> Result<Vec<PathBuf>>
where
    F: Fn(PathBuf) -> bool,
{
    let mut output = vec![];

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if !entry.metadata()?.is_file() {
            continue;
        }

        if (f)(entry.path()) {
            output.push(entry.path());
        }
    }
    output.sort();
    Ok(output)
}

/// Get the file stem (filename without the extension)
pub fn file_stem_as_str(p: &PathBuf) -> Option<&str> {
    p.file_stem().and_then(|p| p.to_str())
}
