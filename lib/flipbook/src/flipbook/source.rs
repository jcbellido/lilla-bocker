#![allow(dead_code)]

// Taking some ideas from logSeq: #lillaOrd-flipbook-compiler
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::common::{FilePath, LanguageCode, MetadataVersion, RawString};

/// This structure points at the idea that audio is secondary to text
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Asset {
    pub text: RawString,
    pub audio: Option<Audio>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PageText(pub HashMap<LanguageCode, Asset>);

impl PageText {
    pub fn texts(&self) -> Vec<(LanguageCode, RawString)> {
        self.0
            .iter()
            .map(|(k, v)| (k.clone(), v.text.clone()))
            .collect()
    }

    pub fn audios(&self) -> Vec<(LanguageCode, FilePath)> {
        let mut answer = vec![];
        self.0.iter().for_each(|(k, v)| {
            if let Some(audio) = &v.audio {
                answer.push((k.clone(), audio.path.clone()));
            }
        });
        answer
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Image {
    pub path: FilePath,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Audio {
    pub path: FilePath,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SourcePage {
    pub background: Image,
    // The cover page for example has no text at all
    pub text: Option<PageText>,
}

/// Root V1 source structure
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FlipbookSource {
    pub version: MetadataVersion,
    pub languages: Vec<LanguageCode>,
    pub default_language: LanguageCode,

    pub title: PageText,
    pub summary: PageText,
    pub miniature: Image,

    pub pages: Vec<SourcePage>,
}

impl FlipbookSource {
    pub fn pages_text(&self) -> Vec<(usize, LanguageCode, RawString)> {
        let mut answer = vec![];
        for (pos, page) in self.pages.iter().enumerate() {
            if let Some(texts_in_page) = &page.text {
                for tip in texts_in_page.texts() {
                    answer.push((pos, tip.0, tip.1));
                }
            }
        }
        answer
    }

    pub fn pages_audios(&self) -> Vec<(usize, LanguageCode, FilePath)> {
        let mut answer = vec![];
        for (pos, page) in self.pages.iter().enumerate() {
            if let Some(texts_in_page) = &page.text {
                for aip in texts_in_page.audios() {
                    answer.push((pos, aip.0, aip.1));
                }
            }
        }
        answer
    }
}
