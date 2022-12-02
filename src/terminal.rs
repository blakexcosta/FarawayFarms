use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf
}

pub fn run_terminal() {
    let args = Cli::parse();
}