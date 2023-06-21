use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to directory you want served
    #[arg(short, long)]
    pub serve: String,

    #[arg(short, long, default_value_t = 8888)]
    pub port: u16,
}
