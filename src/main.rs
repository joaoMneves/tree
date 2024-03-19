use clap::Parser;
use std::path::PathBuf;
use tree::show_dirs;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// path to tree
    #[arg(default_value_t = String::from("./"))]
    path: String,
    /// determines wether files will be show
    #[arg(short, long)]
    show_files: Option<bool>,
    /// the max of recursion
    #[arg(long, short)]
    max_indent: Option<u16>,
}

fn main() {
    let cli = Cli::parse();

    let show_files = {
        match cli.show_files {
            None => false,
            Some(op) => op,
        }
    };

    show_dirs(PathBuf::from(cli.path), show_files, 0, cli.max_indent);
}
