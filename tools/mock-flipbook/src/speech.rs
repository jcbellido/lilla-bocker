// Particularly when run on macos it's possible to use the command "say" to generate TTS on demand
//   the submodules here: english, spanish, swedish, contain samples of sentences generated through
//   chatGPT. The very simple idea is to chain say + format work to generate sample audio files for
//   the mocked flipbooks.
// The man page for say is here: https://ss64.com/osx/say.html and includes some options for the
//   output of the generated voice.
//   let's make some tests under `test_output`
// I seem to have installed in this machine `ffmpeg` which could do the job of transforming the audio
//   files into a format that can be used by the flipbook.

use std::process::Command;

use anyhow::Result;

use crate::args::Language;
use crate::text::english;
use crate::text::spanish;
use crate::text::swedish;

#[cfg(not(target_os = "macos"))]
fn tts(language: Language, path: &str) -> Result<()> {
    tracing::warn!("TTS feature is supported only on macOS");
    Ok(())
}

/// Generate the voice samples included in the corresponding module
#[cfg(target_os = "macos")]
fn tts(language: Language, path: &str) -> Result<()> {
    let (voice, samples) = match language {
        Language::English => (english::VOICE, english::SAMPLES),
        Language::Spanish => (spanish::VOICE, spanish::SAMPLES),
        Language::Swedish => (swedish::VOICE, swedish::SAMPLES),
    };

    tracing::info!(
        "Generating {} samples with `{}` for `{:#?}` under `{}`",
        samples.len(),
        voice,
        language,
        path
    );

    for (index, sample) in samples.iter().enumerate() {
        let raw_filename = format!("{:#?}_{:03}.aiff", language, index);
        let path_raw = std::path::Path::new(path).join(raw_filename);

        tracing::info!("Generating {:#?}", path_raw);

        let output = Command::new("say")
            .arg("--voice")
            .arg(voice)
            .arg("--output-file")
            .arg(path_raw)
            .arg(sample)
            .output()?;

        if !output.status.success() {
            let stderr = std::str::from_utf8(output.stderr.as_slice())?.to_string();
            return Err(anyhow::anyhow!(stderr));
        }
    }
    Ok(())
}

fn reencode(path: &str) -> Result<()> {
    let entries = std::fs::read_dir(path)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("aiff") {
            tracing::debug!("Processing `{:#?}`", path);
            // ffmpeg -i Swedish_000.aiff -vn -acodec libvorbis  Swedish_000.ogg
            let mut new_path = path.clone();
            new_path.set_extension(crate::generator_constants::SPEECH_EXT);

            let output = Command::new("ffmpeg")
                .arg("-y")
                .arg("-i")
                .arg(path)
                .arg("-vn")
                .arg("-acodec")
                .arg("libvorbis")
                .arg(new_path)
                .output()?;

            if !output.status.success() {
                let stderr = std::str::from_utf8(output.stderr.as_slice())?.to_string();
                return Err(anyhow::anyhow!(stderr));
            }
        }
    }

    Ok(())
}

pub fn generate_all_languages(path: &str) -> Result<()> {
    tracing::debug!("generating all languages: {}", path);
    tracing::trace!("creating dir tree: {}", path);
    std::fs::create_dir_all(path)?;
    tts(Language::English, path)?;
    tts(Language::Spanish, path)?;
    tts(Language::Swedish, path)?;
    reencode(path)?;
    Ok(())
}

#[allow(dead_code)]
pub fn generate_language(language: Language, path: &str) -> Result<()> {
    tracing::debug!("generate_language: {:#?} -> {}", language, path);
    tracing::trace!("creating dir tree: {}", path);
    std::fs::create_dir_all(path)?;
    tts(language, path)?;
    reencode(path)?;
    Ok(())
}
