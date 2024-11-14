use std::env;
use std::fs;
use std::io;

// Minimal
fn list_directory_minimal(path: &str) -> io::Result<()> {
    let contents = fs::read_dir(path)?;

    for content in contents {
        let content = content?;
        println!("{}", content.path().display());
    }

    Ok(())
}

/**
 * Lists directory contents
 * 
 * takes arguments
 */
fn main() {

    let args: Vec<String> = env::args().collect();
    let mut path = "./";

    if args.len() > 1 {
        path = args[1].as_str();
    }
    
    if let Err(e) = list_directory_minimal(path) {
        eprintln!("Error reading directory: {}", e);
    }
}
