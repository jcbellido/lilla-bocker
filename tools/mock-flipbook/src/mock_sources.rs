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
// // this

// use std::path::PathBuf;

// // I need to generate these things
// use flipbook::flipbook::source::FlipbookSource;

// // And then I need to call this function
// use flipbook::compile::compile;

// // to obtain a compiled file under the directory

// use anyhow::Result;

// pub struct MockCatalog {
//     pub _placeholder: bool,
//     pub images: Vec<PathBuf>,
//     pub texts: Vec<(String, Option<audio_file?>)>
//     pub texts_by_lang: crate::args::Language, texts?
// }

// impl MockCatalog {
//     pub fn new() -> Result<Self> {
//         unimplemented!()
//     }

//     pub fn generate_v1(&self) -> FlipbookSource {
//         unimplemented!()
//     }
// }
