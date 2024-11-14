use std::collections::HashSet;
use std::env::{self};
use std::file;
use std::fs;
use std::fs::File;
use std::fs::Permissions;
use std::io;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process;

// https://doc.rust-lang.org/std/fs/fn.set_permissions.html

/// Handy function for testing
/// It prints the type of a variable
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn main() -> std::io::Result<()> {
    // Collect the arguments
    let args: Vec<String> = env::args().collect();

    // If there are no arguments, print the help message
    if args.len() <= 1 {
        println!(
            "
            Create file if it does not already exist.
            Usage: create_file [options] [file_name]
            Options:
                -h, --help: Display this help message.
                -p [permissions]: Set the permissions for the file.
                -P gets current file permissions."
        );
    } else {
        let file_name = args[1].clone();
        let current_dir = env::current_dir()?;
        let full_path = current_dir.join(&file_name);

        if !full_path.exists() {
            let mut file = File::create(&file_name)?;
            print!("0");
        } else {
            print!("1");
            return Ok(());
        }
    }
    Ok(())
}
