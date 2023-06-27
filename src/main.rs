use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use clap::Parser;

/// Create a mvn project from cli
#[derive(Parser)]
struct Cli {
    /// The project root to build the mvn project
    project_root: String,
}

fn main() {
    let args = Cli::parse();
    println!("{}", args.project_root);

    match fs::create_dir_all(&args.project_root) {
        Err(e) => panic!("couldn't create dir with error {}", e),
        Ok(_) => println!("created dir with name {}", args.project_root),
    }

    let project_root = args.project_root + "/pom.xml";
    let target_path = Path::new(&project_root);
    let mut file = match File::create(&target_path) {
        Err(e) => panic!("couldn't create file with error {}", e),
        Ok(file) => file,
    };

    match file.write_all("hello world!".as_bytes()) {
        Err(e) => panic!("couldn't write to file with error {}", e),
        Ok(_) => println!("wrote to file successfully!"),
    };
}
