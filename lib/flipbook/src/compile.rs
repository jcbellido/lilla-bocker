use anyhow::Result;

mod build;
mod common;
mod persistence;

use crate::flipbook::source::FlipbookSource;

use common::Arguments;

/// Transform a source flipbook into an artifact consumable by the client and possible to serve.
/// The compiled (or packaged) version of a flipbook is a combination of 2 artifacts
///   1. Metadata
///   2. A binary blob
/// Ideally the source flipbook has been sanitized by doing sanity checks such as:
///    the file exists and the expected texts are there
pub fn compile(source: &FlipbookSource, path_metadata: &str, path_binary: &str) -> Result<()> {
    let args = Arguments {
        source,
        path_metadata,
        path_binary,
    };

    let cr = build::build(&args)?;
    persistence::to_disk(&cr, path_metadata, path_binary)
}
