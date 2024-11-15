extern crate libc;
use std::env;
use std::fs;
use std::io;
use std::os::linux::fs::MetadataExt;
use std::path::Path;
use libc::{S_IFSOCK, S_IFLNK, S_IFREG, S_IFDIR, S_IFCHR, S_IFBLK, S_IFIFO};

/**
 * Colors for color coding and what not
 */
#[derive(Debug)]
pub enum PrettyPrint {
    Reset,
    Bold,
    Underline,
    
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    
    BgBlack,
    BgRed,
    BgGreen,
    BgYellow,
    BgBlue,
    BgMagenta,
    BgCyan,
    BgWhite,
    
    BgBrightBlack,
    BgBrightRed,
    BgBrightGreen,
    BgBrightYellow,
    BgBrightBlue,
    BgBrightMagenta,
    BgBrightCyan,
    BgBrightWhite,
}

/**
 * Implement the pretty print enum 
 */
impl PrettyPrint {
    /**
     *  Get the corresponding ANSI escape code for each enum variant
     */
    fn ansi_code(&self) -> &'static str {
        match *self {
            PrettyPrint::Reset => "\x1b[0m",
            PrettyPrint::Bold => "\x1b[1m",
            PrettyPrint::Underline => "\x1b[4m",
            
            PrettyPrint::Black => "\x1b[30m",
            PrettyPrint::Red => "\x1b[31m",
            PrettyPrint::Green => "\x1b[32m",
            PrettyPrint::Yellow => "\x1b[33m",
            PrettyPrint::Blue => "\x1b[34m",
            PrettyPrint::Magenta => "\x1b[35m",
            PrettyPrint::Cyan => "\x1b[36m",
            PrettyPrint::White => "\x1b[37m",
            
            PrettyPrint::BrightBlack => "\x1b[90m",
            PrettyPrint::BrightRed => "\x1b[91m",
            PrettyPrint::BrightGreen => "\x1b[92m",
            PrettyPrint::BrightYellow => "\x1b[93m",
            PrettyPrint::BrightBlue => "\x1b[94m",
            PrettyPrint::BrightMagenta => "\x1b[95m",
            PrettyPrint::BrightCyan => "\x1b[96m",
            PrettyPrint::BrightWhite => "\x1b[97m",
            
            PrettyPrint::BgBlack => "\x1b[40m",
            PrettyPrint::BgRed => "\x1b[41m",
            PrettyPrint::BgGreen => "\x1b[42m",
            PrettyPrint::BgYellow => "\x1b[43m",
            PrettyPrint::BgBlue => "\x1b[44m",
            PrettyPrint::BgMagenta => "\x1b[45m",
            PrettyPrint::BgCyan => "\x1b[46m",
            PrettyPrint::BgWhite => "\x1b[47m",
            
            PrettyPrint::BgBrightBlack => "\x1b[100m",
            PrettyPrint::BgBrightRed => "\x1b[101m",
            PrettyPrint::BgBrightGreen => "\x1b[102m",
            PrettyPrint::BgBrightYellow => "\x1b[103m",
            PrettyPrint::BgBrightBlue => "\x1b[104m",
            PrettyPrint::BgBrightMagenta => "\x1b[105m",
            PrettyPrint::BgBrightCyan => "\x1b[106m",
            PrettyPrint::BgBrightWhite => "\x1b[107m",
        }
    }

    /**
     * Apply a style and print text
     */
    pub fn pretty_print(&self, text: &str) {
        print!("{}{}{}", self.ansi_code(), text, PrettyPrint::Reset.ansi_code());
    }

    /**
     * Apply a style and print line of text
     */
    pub fn pretty_println(&self, text: &str) {
        println!("{}{}{}", self.ansi_code(), text, PrettyPrint::Reset.ansi_code());
    }

}

/**
 * Enum for file types
 */
#[derive(Debug)]
enum FileType {
    Directory,
    SymbolicLink,
    Normal,
    Sock,
    Whiteout,
    ArgDirectory,
    Fifo,
    Chardev,
    Blockdev,
    Unknown
}

impl FileType {

    /**
     * Match metadata to file type
     */
    pub fn match_metadata(path_to_file: &Path) -> FileType {
        match fs::metadata(path_to_file) {
            Ok(metadata) => {
                    let mode = metadata.st_mode();
        
                    // #TODO: Implement whitespace and arg directory detections
                    if Self::is_socket(mode) {
                        return Self::Sock // Socket
                    } else if Self::is_symlink(mode) {
                        return Self::SymbolicLink // Symbolic Link
                    } else if Self::is_regular_file(mode) {
                        return Self::Normal // Regular / Normal
                    } else if Self::is_directory(mode) {
                        return Self::Directory // Directory
                    } else if Self::is_character_device(mode) {
                        return Self::Chardev // Character device
                    } else if Self::is_block_device(mode) {
                        return Self::Blockdev // Block device
                    } else if Self::is_fifo(mode) {
                        return Self::Fifo // Fifo (named pipe)
                    } else {
                        return Self::Unknown // Unknown (For everything else)
                    }
                }
            Err(e) => {
                println!("ERR: cannot read metadata of path {:?} suffering error: {}", path_to_file, e);
            }
        }

        return FileType::Unknown
    }

    /**
     * Return a string corrosponding to the type
     */
    pub fn get_type_short_name(&self) -> &'static str {
        let mut return_string = "";
        match *self {
            Self::Directory => {
                return_string = "DIR"
            },
            Self::SymbolicLink => {
                return_string = "SLNK"
            },
            Self::Normal => {
                return_string = "NRML"
            },
            Self::Sock => {
                return_string = "SOCK"
            },
            Self::Whiteout => {
                return_string = "WTOT"
            },
            Self::ArgDirectory => {
                return_string = "ADIR"
            },
            Self::Fifo => {
                return_string = "FIFO"
            },
            Self::Chardev => {
                return_string = "CRDV"
            },
            Self::Blockdev => {
                return_string = "BKDV"
            },
            Self::Unknown => {
                return_string = "UKWN"
            }
        }

        return return_string;
    }

    // helper functions

    /**
     * Determine if it is a socket
     */
    fn is_socket(mode: libc::mode_t) -> bool {
        mode & libc::S_IFMT == S_IFSOCK
    }
    
    /**
     * Determine if it is a symlink
     */
    fn is_symlink(mode: libc::mode_t) -> bool {
        mode & libc::S_IFMT == S_IFLNK
    }
    
    /**
     * Determine if it is a regular file type
     */
    fn is_regular_file(mode: libc::mode_t) -> bool {
        mode & libc::S_IFMT == S_IFREG
    }
    
    /**
     * Determine if it is a directory
     */
    fn is_directory(mode: libc::mode_t) -> bool {
        mode & libc::S_IFMT == S_IFDIR
    }
    
    /**
     * Determine if it is a character device
     */
    fn is_character_device(mode: libc::mode_t) -> bool {
        mode & libc::S_IFMT == S_IFCHR
    }
    
    /**
     * Determine if it is a block device
     */
    fn is_block_device(mode: libc::mode_t) -> bool {
        mode & libc::S_IFMT == S_IFBLK
    }
    
    /**
     * Determine if it is a named pipe
     */
    fn is_fifo(mode: libc::mode_t) -> bool {
        mode & libc::S_IFMT == S_IFIFO
    }

    // #TODO: implement whiteout and arg directory detection helper functions
}

/**
 * Define standard output 
 */
struct StandardOutput {
    file_type: String,
    file_name: String,
    file_information: String
}

/**
 * Implement standard output
 */
impl StandardOutput {

    /**
     * Return the non-pretty print version of the string.
     */
    pub fn get_npp_text(&self) -> String{
        let mut output_string: String = "".to_string();
        output_string.push_str(&self.file_type);
        output_string.push_str(" ");
        output_string.push_str(&self.file_name);
        output_string.push_str(" ");
        output_string.push_str(&self.file_information);
        return output_string;
    }
}

/**
 * Minimal listing
 */
fn list_directory_minimal(path: &str) -> io::Result<()> {
    let contents = fs::read_dir(path)?;

    for content in contents {
        let content = content?;
        let content_to_match = content.path();
        let content_type = FileType::get_type_short_name(&FileType::match_metadata(&content_to_match));
        print!("[{} {}]  ", content_type, content.path().display());
    }

    Ok(())
}

/**
 * Print help text
 */
fn help_text() {
    println!("");
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

    println!();
}