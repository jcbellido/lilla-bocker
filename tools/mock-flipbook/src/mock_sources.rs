// // Starting from the path output
// // Having a predefined namne for the folders -> I'm missing a config object here
// // Gather every single file
// //    images is an array of names and a method to pick one at random
// //    texts / tts: language - number
// // There's an issue with the language mapping (?)
// //   I have a predefined collection of langs: en, es, se
// //   that have both texts and audios
// // Let's keep in mind that images and texts are, at the moment, disconnected

// // Agreements: this particular audio books will have all 3 audios

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use anyhow::Ok;
use anyhow::Result;
use strum::IntoEnumIterator;

use crate::args::Language;
use crate::generator_constants;
// // I need to generate these things
// use flipbook::flipbook::source::FlipbookSource;

// // And then I need to call this function
// use flipbook::compile::compile;

// // to obtain a compiled file under the directory

type LinesByLanguage = HashMap<Language, Vec<Line>>;

#[derive(Default, Clone, Debug)]
pub struct Line {
    pub text: String,
    pub tts: Option<PathBuf>,
}

#[derive(Clone, Debug)]
pub struct MockCatalog {
    pub images: Vec<PathBuf>,
    pub lines_by_language: LinesByLanguage,
}

impl MockCatalog {
    pub fn new(path: &str) -> Result<Self> {
        let images = MockCatalog::gather_images(path)?;
        let lines_by_language = MockCatalog::gather_lines(path)?;
        Ok(MockCatalog {
            images,
            lines_by_language,
        })
    }

    fn gather_lines(path: &str) -> Result<LinesByLanguage> {
        let mut lbl = LinesByLanguage::default();
        // Iterate per available language
        for language in Language::iter() {
            // collect all the files for this particular language and match them.
            lbl.insert(
                language.clone(),
                MockCatalog::gather_lines_for_lang(path, language)?,
            );
        }
        Ok(lbl)
    }

    fn gather_lines_for_lang(path: &str, lang: Language) -> Result<Vec<Line>> {
        let prefix_lang = format!("{:#?}", lang);
        let speech_dir = std::path::Path::new(path).join(generator_constants::DIR_SPEECH);
        let speech_files = get_files_from_dir(speech_dir, |de_path| {
            let Some(ext) = de_path.extension() else {
                return false;
            };
            if ext != generator_constants::SPEECH_EXT {
                return false;
            }
            let Some(basename) = de_path.file_name() else {
                return false;
            };
            let Some(basename) = basename.to_str() else {
                return false;
            };
            basename.starts_with(&prefix_lang)
        })?;
        let text_dir = std::path::Path::new(path).join(generator_constants::DIR_TEXTS);
        let text_files = get_files_from_dir(text_dir, |de_path| {
            let Some(ext) = de_path.extension() else {return false;};
            if ext != generator_constants::TEXTS_EXT {
                return false;
            }
            let Some(basename) = de_path.file_name() else {return false; };
            let Some(basename) = basename.to_str() else { return false; };
            basename.starts_with(&prefix_lang)
        })?;
        let mut output = vec![];

        for path_text_file in text_files {
            let reader = std::fs::File::open(&path_text_file)?;
            let line: crate::text::Line = serde_json::from_reader(reader)?;

            let Some(t_base) = path_text_file.file_stem() else {continue;};
            let Some(t_base) = t_base.to_str() else {continue;};
            let t_base = t_base.replace(&prefix_lang, "");
            match speech_files.iter().find(|sf| {
                let Some(s_base) = sf.file_stem() else {return false;};
                let Some(s_base) = s_base.to_str() else {return false;};
                s_base.ends_with(&t_base)
            }) {
                Some(speech_file) => output.push(Line {
                    text: line.text,
                    tts: Some(speech_file.clone()),
                }),
                None => output.push(Line {
                    text: line.text,
                    tts: None,
                }),
            }
        }
        Ok(output)
    }

    fn gather_images(path: &str) -> Result<Vec<PathBuf>> {
        let dir_images = std::path::Path::new(path).join(generator_constants::DIR_IMAGES);

        Ok(get_files_from_dir(dir_images, |de_path| {
            let Some(ext) = de_path.extension() else {
                return false;
            };
            ext == crate::generator_constants::IMAGES_EXT
        })?)
    }
}

fn get_files_from_dir<F>(path: PathBuf, f: F) -> Result<Vec<PathBuf>>
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

#[cfg(test)]
mod tests {
    use crate::args::Language;
    use crate::mock_sources::MockCatalog;

    use strum::EnumCount;

    const PATH_SOURCE: &str = "./test_output";

    #[test]
    fn gather_images() {
        let images = MockCatalog::gather_images(PATH_SOURCE).unwrap();

        assert_eq!(images.len(), 256 as usize);
    }

    #[test]
    fn gather_lines() {
        let lines = MockCatalog::gather_lines(PATH_SOURCE).unwrap();
        assert_eq!(lines.len(), Language::COUNT);
    }

    #[test]
    fn gather_lines_for_language() {
        let lines_in_spanish =
            MockCatalog::gather_lines_for_lang(PATH_SOURCE, Language::Spanish).unwrap();
        assert_eq!(lines_in_spanish.len(), 35 as usize);
        let line = lines_in_spanish.get(2).unwrap();
        assert!(line.tts.is_some());
    }

    #[test]
    fn build_mock_catalog() {
        let catalog = MockCatalog::new(PATH_SOURCE).unwrap();
        assert_eq!(catalog.lines_by_language.len(), 3 as usize);
        assert_eq!(catalog.images.len(), 256 as usize);
    }
}
