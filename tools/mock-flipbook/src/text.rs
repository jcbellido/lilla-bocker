use anyhow::Result;

use serde::{Deserialize, Serialize};

use crate::args::Language;

pub mod english;
pub mod spanish;
pub mod swedish;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Line {
    pub text: String,
}

pub fn generate_texts(path: &str) -> Result<()> {
    tracing::debug!("generating texts into: {}", path);
    tracing::trace!("creating dir tree: {}", path);
    std::fs::create_dir_all(path)?;
    generate_lang(Language::English, path)?;
    generate_lang(Language::Spanish, path)?;
    generate_lang(Language::Swedish, path)?;
    Ok(())
}

fn generate_lang(language: Language, path: &str) -> Result<()> {
    let (_voice, samples) = match language {
        Language::English => (english::VOICE, english::SAMPLES),
        Language::Spanish => (spanish::VOICE, spanish::SAMPLES),
        Language::Swedish => (swedish::VOICE, swedish::SAMPLES),
    };

    for (index, sample) in samples.iter().enumerate() {
        let raw_filename = format!("{:#?}_{:03}.json", language, index);
        let path_raw = std::path::Path::new(path).join(raw_filename);

        let line = Line {
            text: sample.to_string(),
        };

        let f = std::fs::File::create(path_raw)?;
        serde_json::to_writer(f, &line)?;
    }
    Ok(())
}
