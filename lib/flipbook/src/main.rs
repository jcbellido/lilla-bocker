#![deny(clippy::all)]
#![warn(clippy::nursery, clippy::pedantic)]

use std::path::Path;

use anyhow::Result;

mod compile;
mod flipbook;

use crate::flipbook::source::FlipbookSource;

// Let's nail a format for the files
pub fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    tracing::info!("Hardcoded flipbook compilation example");

    // Let's use the file `sample_source.json`
    let path = "./test_source/sample_source.json";
    tracing::info!("Reading from: {}", path);
    let p = Path::new(path);
    assert!(p.exists());

    let content = std::fs::read_to_string(path)?;

    let source: FlipbookSource = serde_json::from_str(&content)?;

    let path_metadata = "./test_output/compiled_sample_source.json";
    let path_binary = "./test_output/compiled_sample_source.bin";

    compile::compile(&source, path_metadata, path_binary)?;

    tracing::info!("Metadata generated: {}", path_metadata);
    tracing::info!("Binary generated: {}", path_binary);
    Ok(())
}
