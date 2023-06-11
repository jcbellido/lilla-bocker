/// Identifies the version of the metadata file. Right now I imagine this as matching
///   between the compiled version and the source version.
pub type MetadataVersion = u16;

/// This is a "true string" something the reader will see
pub type RawString = String;

/// As stupidly simple as possible, even an enum, perhaps?
pub type LanguageCode = String;

pub type FilePath = String;
