/*
This app creates a file given by console arguments &
copies makra.rs to the new file.
*/
use clap::Parser;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(author = "Arakaki", version = "1.0.0", about, long_about = None)]
struct Cli {
    #[arg(long)]
    make: String,
    // #[arg(long)]
    // copy: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let home = env::var("HOME").expect("$HOME is not set");
    let makra_path = &format!("{}/.config/makra/src/makra.rs", home);

    match Path::new(makra_path).is_file() {
        true => {
            let mut new_file = cli.make;
            if !new_file.contains(".rs") {
                new_file.push_str(".rs");
            }
            fs::copy(makra_path, new_file).unwrap();
        }

        false => {
            println!("shit");
        }
    }
}
