use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use anyhow::Ok;
use anyhow::Result;
use flipbook::flipbook::source;
use rand::seq::IteratorRandom;
use rand::seq::SliceRandom;
use rand::thread_rng;
use strum::IntoEnumIterator;

use crate::args::{Language, PageRange};
use crate::generator_constants;

#[derive(Default, Clone, Debug)]
pub struct Line {
    pub text: String,
    pub tts: Option<PathBuf>,
}

/// Every translated version of the same line
type LocLine = HashMap<Language, Line>;

#[derive(Clone, Debug)]
pub struct MockCatalog {
    pub images: Vec<PathBuf>,
    pub localized_lines: Vec<LocLine>,
}

impl MockCatalog {
    pub fn new(path: &str) -> Result<Self> {
        let images = MockCatalog::gather_images(path)?;
        let localized_lines = MockCatalog::gather_loc_lines(path)?;
        Ok(MockCatalog {
            images,
            localized_lines,
        })
    }

    pub fn get_metadata_version(&self) -> flipbook::flipbook::common::MetadataVersion {
        1
    }

    pub fn get_default_lang(&self) -> flipbook::flipbook::common::LanguageCode {
        let v: Vec<Language> = Language::iter().map(|i| i).collect();
        let mut rng = thread_rng();
        let lang = v.choose(&mut rng).expect("Should have languages, yo");
        format!("{:#?}", lang)
    }

    pub fn get_languages(&self) -> Vec<flipbook::flipbook::common::LanguageCode> {
        Language::iter().map(|i| format!("{:#?}", i)).collect()
    }

    pub fn get_image(&self) -> Option<source::Image> {
        let mut rng = thread_rng();
        if let Some(pb) = self.images.choose(&mut rng) {
            let Some(pb) = pb.to_str() else { return None;};
            Some(source::Image {
                path: pb.to_string(),
            })
        } else {
            None
        }
    }

    pub fn get_image_n(&self, page_n: usize) -> Option<source::Image> {
        if let Some(pb) = self.images.get(page_n) {
            let Some(pb) = pb.to_str() else { return None;};
            Some(source::Image {
                path: pb.to_string(),
            })
        } else {
            None
        }
    }

    pub fn get_page_text(&self) -> Option<source::PageText> {
        let mut rng = thread_rng();
        if let Some(line) = self.localized_lines.choose(&mut rng) {
            let mut pt: source::PageText = source::PageText::default();
            for (k, v) in line {
                let lang = format!("{:#?}", k);
                let asset = source::Asset {
                    text: v.text.clone(),
                    audio: match &v.tts {
                        Some(pbt) => {
                            if let Some(pbt) = pbt.to_str() {
                                Some(source::Audio {
                                    path: pbt.to_string(),
                                })
                            } else {
                                None
                            }
                        }
                        None => None,
                    },
                };
                pt.0.insert(lang, asset);
            }
            Some(pt)
        } else {
            None
        }
    }

    pub fn get_source_page_n(&self, page_n: usize) -> source::SourcePage {
        let background = self.get_image_n(page_n).unwrap();
        let text = self.get_page_text();

        source::SourcePage { background, text }
    }

    pub fn build_flipbook(&self, page_range: &PageRange) -> source::FlipbookSource {
        let mut rng = thread_rng();
        let r = page_range.min..page_range.max;
        let num_pages = r.choose(&mut rng).expect("Pages should return something");

        let r = 0..num_pages;

        let built_pages: Vec<source::SourcePage> =
            r.map(|i| self.get_source_page_n(i as usize)).collect();

        source::FlipbookSource {
            version: self.get_metadata_version(),
            languages: self.get_languages(),
            default_language: self.get_default_lang(),
            title: self
                .get_page_text()
                .expect("No page text found? Check sources"),
            summary: self
                .get_page_text()
                .expect("No page text found? Check sources"),
            miniature: self.get_image().expect("No image found? Check sources"),
            pages: built_pages,
        }
    }
}

impl MockCatalog {
    fn gather_loc_lines(path: &str) -> Result<Vec<LocLine>> {
        // With all langs
        //   Join all by fake SID into a `LocLine` then push into `output`
        // Iterate per available language
        let mut all_loc_lines: Vec<(Language, HashMap<String, Line>)> = Language::iter()
            .map(|l| (l.clone(), MockCatalog::gather_loc_lines_lang(path, l)))
            .filter(|p| p.1.is_ok())
            .map(|p| (p.0, p.1.unwrap()))
            .collect();

        if all_loc_lines.is_empty() {
            anyhow::bail!("Lines empty?");
        }

        let mut output = vec![];

        let (main_lang, mut main_lines) = all_loc_lines.pop().unwrap();
        for (m_sid, m_line) in main_lines.drain() {
            let mut current_line: HashMap<Language, Line> = HashMap::default();
            current_line.insert(main_lang.clone(), m_line);

            // And now the other members of all_loc_lines
            all_loc_lines.iter().for_each(|(lang, lang_lines)| {
                if let Some(l) = lang_lines.get(&m_sid) {
                    current_line.insert(lang.clone(), l.clone());
                }
            });

            output.push(current_line);
        }
        Ok(output)
    }

    fn gather_loc_lines_lang(path: &str, lang: Language) -> Result<HashMap<String, Line>> {
        // Per lang
        //   list all speech files -> get their fake SID -> HashMap<fake SID, PathBuff>
        //   list all text files -> get their fake SID -> HashMap<fake SID, Text>
        //   Join all text and all speech -> Language available assets -> HashMap<fake SID, Line>
        let lang_prefix = format!("{:#?}", lang);
        let speech_dir = std::path::Path::new(path).join(generator_constants::DIR_SPEECH);
        let speech_files = get_files_from_dir(speech_dir, |de_path| {
            match_loc_name_ext(&de_path, &lang_prefix, generator_constants::SPEECH_EXT)
        })?;

        let mut map_speech: HashMap<String, PathBuf> = HashMap::default();
        for sf in speech_files {
            let Some(fake_sid) = get_fake_string_id(&sf, &lang_prefix) else { continue;};
            map_speech.insert(fake_sid, sf);
        }

        let text_dir = std::path::Path::new(path).join(generator_constants::DIR_TEXTS);
        let text_files = get_files_from_dir(text_dir, |de_path| {
            match_loc_name_ext(&de_path, &lang_prefix, generator_constants::TEXTS_EXT)
        })?;
        let mut map_text: HashMap<String, String> = HashMap::default();
        for tf in text_files {
            let Some(fake_sid) = get_fake_string_id(&tf, &lang_prefix) else { continue;};
            let reader = std::fs::File::open(&tf)?;
            let line: crate::text::Line = serde_json::from_reader(reader)?;
            map_text.insert(fake_sid, line.text);
        }

        let mut output = HashMap::default();
        for (fake_sid, text) in map_text {
            let tts = map_speech.remove(&fake_sid);
            output.insert(fake_sid, Line { text, tts });
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

fn get_fake_string_id(p: &PathBuf, lang_prefix: &str) -> Option<String> {
    file_stem_as_str(p).and_then(|fs| Some(fs.replace(lang_prefix, "")))
}

/// Get the file stem (filename without the extension)
fn file_stem_as_str(p: &PathBuf) -> Option<&str> {
    p.file_stem().and_then(|p| p.to_str())
}

/// Compares the filename against [lang_prefix].*\.[extension]
fn match_loc_name_ext(p: &PathBuf, lang_prefix: &str, extension: &str) -> bool {
    let Some(f_ext) = p.extension() else {
                return false;
            };
    if f_ext != extension {
        return false;
    }
    let Some(basename) = p.file_name() else {
                return false;
            };
    let Some(basename) = basename.to_str() else {
                return false;
            };
    basename.starts_with(&lang_prefix)
}

/// Lists a directory files without recursing
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
    use strum::EnumCount;

    use crate::{args::Language, mock_sources::MockCatalog};
    const PATH_SOURCE: &str = "./test_output";

    #[test]
    fn gather_images() {
        #[cfg(not(target_os = "macos"))]
        return;
        let images = MockCatalog::gather_images(PATH_SOURCE).unwrap();
        assert_eq!(images.len(), 256 as usize);
    }

    #[test]
    fn build_mock_catalog() {
        #[cfg(not(target_os = "macos"))]
        return;
        let catalog = MockCatalog::new(PATH_SOURCE).unwrap();
        assert_eq!(catalog.images.len(), 256 as usize);
    }

    #[test]
    fn get_image() {
        #[cfg(not(target_os = "macos"))]
        return;
        let catalog = MockCatalog::new(PATH_SOURCE).unwrap();
        let pt = catalog.get_image();
        println!(">> {:#?}", pt);
        assert!(pt.is_some());
    }

    #[test]
    fn get_page_text() {
        #[cfg(not(target_os = "macos"))]
        return;
        let catalog = MockCatalog::new(PATH_SOURCE).unwrap();
        let pt = catalog.get_page_text();
        println!(">> {:#?}", pt);
        assert!(pt.is_some());
    }

    #[test]
    fn get_default_lang() {
        #[cfg(not(target_os = "macos"))]
        return;
        let catalog = MockCatalog::new(PATH_SOURCE).unwrap();
        let def_lang = catalog.get_default_lang();
        assert!(!def_lang.is_empty());
    }

    #[test]
    fn get_languages() {
        #[cfg(not(target_os = "macos"))]
        return;
        let catalog = MockCatalog::new(PATH_SOURCE).unwrap();
        let langs = catalog.get_languages();
        assert_eq!(langs.len(), Language::COUNT);
    }
}
