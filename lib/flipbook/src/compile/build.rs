use anyhow::Result;
use base64::{engine::general_purpose, Engine};

use crate::flipbook::package::{AudioDB, FilePositionInPackage, FlipbookPackage, TextDB};

use super::common::{Arguments, Artifacts};

pub fn build(args: &Arguments) -> Result<Artifacts> {
    let miniature_data = std::fs::read(&args.source.miniature.path)?;
    let miniature: String = general_purpose::STANDARD.encode(miniature_data);

    let (images_in_pages, mut binary_package) = construct_background_images(args);

    let audio_db = construct_audio_db(args, &mut binary_package);

    let page_texts = construct_text_db(args);

    let title_sid = format!("TITLE_{}", args.source.default_language);
    let summary_sid = format!("SUMMARY_{}", args.source.default_language);

    let metadata = FlipbookPackage {
        version: args.source.version.clone(),
        languages: args.source.languages.clone(),
        default_language: args.source.default_language.clone(),
        binary_package_url: args.binary_file_path().unwrap(),
        texts: page_texts,
        audio: audio_db,
        title: title_sid,
        summary: summary_sid,
        miniature,
        images_in_pages,
    };

    let cr = Artifacts {
        metadata,
        binary_package,
    };
    Ok(cr)
}

fn file_extension(path: &str) -> String {
    std::path::Path::new(path)
        .extension()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

fn append_file_to_binary_package(
    binary_package: &mut Vec<u8>,
    path: &str,
) -> FilePositionInPackage {
    let mut file_content = std::fs::read(&path).unwrap();

    let fip = FilePositionInPackage {
        format: file_extension(&path),
        start: binary_package.len() as u64,
        length: file_content.len() as u64,
    };

    binary_package.append(&mut file_content);
    fip
}

fn construct_audio_db(args: &Arguments, binary_package: &mut Vec<u8>) -> AudioDB {
    let mut audio_db = AudioDB::default();
    let audios = args.source.pages_audios();
    for a in audios {
        let audio_id = format!("PAGE_{}_{}", a.0.to_string(), a.1);
        let fip = append_file_to_binary_package(binary_package, &a.2);
        audio_db.insert(audio_id, fip);
    }
    audio_db
}

fn construct_background_images(args: &Arguments) -> (Vec<FilePositionInPackage>, Vec<u8>) {
    let mut fpip = vec![];
    let mut binary_package = vec![];

    for p in &args.source.pages {
        let image_path = std::path::Path::new(&p.background.path);
        if !image_path.exists() {
            continue;
        }

        let fip = append_file_to_binary_package(&mut binary_package, &p.background.path);

        fpip.push(fip);
    }

    (fpip, binary_package)
}

/// Traverses the source structure gathering all the texts present, using for stringIDs
/// TITLE_<lang>
/// SUMMARY_<lang>
/// PAGE_<page no.>_<lang>  
fn construct_text_db(args: &Arguments) -> TextDB {
    let mut tdb = TextDB::default();

    let pages_texts = args.source.pages_text();

    for pt in &pages_texts {
        let _ = tdb.insert(format!("PAGE_{}_{}", pt.0.to_string(), pt.1), pt.2.clone());
    }

    for title_texts in &args.source.title.texts() {
        let _ = tdb.insert(format!("TITLE_{}", title_texts.0), title_texts.1.clone());
    }

    for summary_texts in &args.source.summary.texts() {
        let _ = tdb.insert(
            format!("SUMMARY_{}", summary_texts.0),
            summary_texts.1.clone(),
        );
    }

    tdb
}
