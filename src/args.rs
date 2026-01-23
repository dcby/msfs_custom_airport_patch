use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    // /// Path to manifest.json
    // #[arg(short, long, default_value = "manifest.json")]
    // pub file: String,
    // #[arg(num_args = 1..)]
    pub path: String,

    pub next_path: Vec<String>,
}
