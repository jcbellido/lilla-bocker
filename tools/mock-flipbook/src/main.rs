use anyhow::Result;
use clap::Parser;

// use flipbook;
mod args;
mod speech;

use args::Args;

fn main() -> Result<()> {
    let args = Args::parse();

    tracing_subscriber::fmt::init();
    tracing::info!("Flipbook mock generator invoked");
    tracing::debug!("Arguments used: `{:#?}`", args);

    std::fs::create_dir_all(&args.path)?;

    if args.tts {
        let output_path = std::path::Path::new(&args.path).join("tts");
        let output_path = output_path.to_str().unwrap();
        speech::generate_all_languages(output_path)?;
    }

    Ok(())
}
