use clap::Parser;

#[derive(Clone, Debug)]
pub enum Language {
    English,
    Spanish,
    Swedish,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub path: String,

    /// Add this flag to invoke the generation of TTS (will appear under {path}/tts)
    #[arg(short, long, default_value_t = false)]
    pub tts: bool,
}
