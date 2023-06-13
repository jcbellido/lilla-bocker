use clap::{Parser, ValueEnum};

mod image_size;
mod page_range;

pub use self::image_size::ImageSize;
pub use self::page_range::PageRange;

#[derive(Clone, Debug, ValueEnum)]
pub enum Language {
    English,
    Spanish,
    Swedish,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path used to store the raw assets and the generated mock - flipbooks
    #[arg(short, long)]
    pub path: String,

    /// Add this flag to invoke the generation of TTS (will appear under {path}/tts)
    #[arg(short, long, default_value_t = false)]
    pub tts: bool,

    /// When TTS is requested use this arg to restrict the generation to a language
    #[arg(value_enum, short, long)]
    pub lang: Option<Language>,

    /// A range of pages you want on your flipbooks, ie: "16,32"
    #[arg(long, default_value_t = PageRange::default())]
    pub pages: PageRange,

    /// Specify the sizes of the mock images you want, ie: "640,480"
    #[arg(long, default_value_t = ImageSize::default())]
    pub image_size: ImageSize,
}
