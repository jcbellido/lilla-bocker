use std::io::Write;

use anyhow::Result;

use super::common::Artifacts;

pub fn to_disk(compiled: &Artifacts, path_metadata: &str, path_binary: &str) -> Result<()> {
    {
        let f_metadata = std::fs::File::create(path_metadata)?;
        serde_json::to_writer_pretty(f_metadata, &compiled.metadata)?;
    }
    {
        let mut f_bin = std::fs::File::create(path_binary)?;
        f_bin.write_all(&compiled.binary_package)?;
    }
    Ok(())
}
