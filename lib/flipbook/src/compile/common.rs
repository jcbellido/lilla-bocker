use crate::flipbook::package::FlipbookPackage;
use crate::flipbook::source::FlipbookSource;
/// Objects generated through a compilation of the sources (build module and associates)
pub struct Artifacts {
    pub metadata: FlipbookPackage,
    pub binary_package: Vec<u8>,
}
/// How the process was invoked
pub struct Arguments<'a> {
    pub source: &'a FlipbookSource,
    pub path_metadata: &'a str,
    pub path_binary: &'a str,
}

impl<'a> Arguments<'a> {
    fn file_name(path: &'a str) -> Option<String> {
        let as_path = std::path::Path::new(path);
        Some(as_path.file_name()?.to_str()?.to_string())
    }

    pub fn binary_file_path(&self) -> Option<String> {
        Arguments::file_name(self.path_binary)
    }
}
