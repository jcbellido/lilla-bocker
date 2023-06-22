use anyhow::Result;
use clap::Parser;

mod args;
mod generator_constants;
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
        tracing::info!("TTS generation");
        let output_path = std::path::Path::new(&args.path).join(generator_constants::DIR_SPEECH);
        let output_path = output_path.to_str().unwrap();

        if let Some(selected_lang) = &args.lang {
            tracing::info!("Target lang specified {:#?}", selected_lang);
            speech::generate_language(selected_lang.clone(), output_path)?;
        } else {
            speech::generate_all_languages(output_path)?;
        }
    } else {
        tracing::info!("Skipping TTS generation");
    }

    if args.string {
        tracing::info!("String generation");
        let output_path = std::path::Path::new(&args.path).join(generator_constants::DIR_TEXTS);
        text::generate_texts(output_path.to_str().unwrap())?;
    } else {
        tracing::info!("Skipping string generation");
    }

    if args.image {
        tracing::info!("Image generation");
        let output_path = std::path::Path::new(&args.path).join(generator_constants::DIR_IMAGES);
        image::build_images(output_path.to_str().unwrap(), &args.image_size, &args.pages)?;
    } else {
        tracing::info!("Skipping image generation");
    }

    if args.num_flipbooks > 0 {
        tracing::info!("Flipbook generation");
        let catalog = mock_sources::MockCatalog::new(&args.path)?;
        let dir_flipbook =
            std::path::Path::new(&args.path).join(generator_constants::DIR_FLIPBOOKS);
        std::fs::create_dir_all(&dir_flipbook)?;
        for book_number in 0..args.num_flipbooks {
            let path_metadata = dir_flipbook.join(format!("fb_{:03}.json", book_number));
            let path_bin = dir_flipbook.join(format!("fb_{:03}.bin", book_number));

            flipbook::compile::compile(
                &catalog.build_flipbook(&args.pages),
                &path_metadata.to_str().unwrap(),
                &path_bin.to_str().unwrap(),
            )?;
        }
    } else {
        tracing::info!("Skipping flipbook generation");
    }

    Ok(())
}
