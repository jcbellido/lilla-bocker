use anyhow::Result;
use clap::Parser;

mod args;
mod image;
mod mock_sources;
mod speech;
mod text;

use args::Args;

fn main() -> Result<()> {
    let args = Args::parse();

    tracing_subscriber::fmt::init();
    tracing::info!("Flipbook mock generator invoked");
    tracing::debug!("Arguments used: `{:#?}`", args);

    std::fs::create_dir_all(&args.path)?;

    generate_assets(&args)?;

    tracing::info!("Finished, assets generated under `{}`", args.path);
    Ok(())
}

fn generate_assets(args: &Args) -> Result<(), anyhow::Error> {
    if args.tts {
        let output_path = std::path::Path::new(&args.path).join("tts");
        let output_path = output_path.to_str().unwrap();

        if let Some(selected_lang) = &args.lang {
            tracing::info!("Target lang specified {:#?}", selected_lang);
            speech::generate_language(selected_lang.clone(), output_path)?;
        } else {
            speech::generate_all_languages(output_path)?;
        }
    }

    if args.string {
        let output_path = std::path::Path::new(&args.path).join("texts");
        text::generate_texts(output_path.to_str().unwrap())?;
    }

    if args.image {
        let output_path = std::path::Path::new(&args.path).join("images");
        image::build_images(output_path.to_str().unwrap(), &args.image_size, &args.pages)?;
    }

    Ok(())
}
