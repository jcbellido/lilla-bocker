#![allow(dead_code)]

// The `flipbook_package` module specializes on serving the flipbooks online,
//   help developing some of the features of the frontend, optimizing network usage.
// The packaged version of a flipbook contains 2 artifacts:
//   1. A metadata file (probably a .json as of today) that also embeds the image in the miniature
//   2. A binary package with all the binary assets concatenated as a single blob

// Layout of the binary package
// [ Cover, Image page 00, Image page 01, .. Image page N]
// Optional, per page: [Audio page 01, .., Audio page N]

// Taking some ideas from logSeq: #lillaOrd-flipbook-compiler
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::common::{LanguageCode, MetadataVersion, RawString};

type Base64Image = String;
type BinaryPackageURL = String;
/// For the client could be useful to know if the origin is a PNG or a JPEG or a WAV or an OGG
type BinaryFormat = String;

/// To be honest here I'm thinking about a convention such as:
///   ID_00_en -> A line "ID_00" in English
///   ID_00_es -> A line "ID_00" in Spanish
///   ID_00_se -> A LINE "ID_00" in Swedish
pub type StringID = String;

#[allow(non_camel_case_types)]
pub type TextDB = HashMap<StringID, RawString>;

#[allow(non_camel_case_types)]
pub type AudioDB = HashMap<StringID, FilePositionInPackage>;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FilePositionInPackage {
    pub format: BinaryFormat,
    pub start: u64,
    pub length: u64,
}

/// This is the "high level item" that the client will use to allow the user select which flipbook is going to be "played"
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FlipbookPackage {
    /// Version of this data definition (it's possible to predict the format that's incoming)
    pub version: MetadataVersion,
    pub languages: Vec<LanguageCode>,
    pub default_language: LanguageCode,
    // ---- flipbook resources link
    pub binary_package_url: BinaryPackageURL,
    /// See note for StringID but I'm suggesting something like: ID_00_en
    pub texts: TextDB,
    pub audio: AudioDB,

    // ---- This is more "natural data" of the object
    pub title: StringID,
    pub summary: StringID,
    /// This is an embedded image in the payload of the Package metadata. The idea is that when the
    pub miniature: Base64Image,

    /// Images on the pages: they're expected to exist
    pub images_in_pages: Vec<FilePositionInPackage>,
}
