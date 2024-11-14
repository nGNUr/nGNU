use std::env::{self};
use std::fs;
use std::os::unix::fs::DirBuilderExt;

/**
 * Take a path and a permission and create a directory.
 * 
 * @path take an &str path name/name of directory
 * @permissions 
 */
fn create_directory(path: &str, permissions: u32) {

    let mut builder = fs::DirBuilder::new();

    builder.recursive(true);
    builder.mode(permissions);
    
    match builder.create(path) {
        Ok(()) => println!("Successfully created {:?} with permissions {:o}", path, permissions), 
        Err(e) => println!("Error: {:?}", e)
    }
}

/**
 * Prints the help page
 */
fn print_help_menu() {
    println!("
Create a directory if it does not already exist.
Input format: [directory name]<flags + information>
Flags:
    -p | --permissions
        Provide flag and permission set in octal
    -P | --parents 
        Provide a list of parent directories. If they need to have special permissions, 
        use -pp or --parent_permissions.
        This modifies the input format to [directory name]<flags + information> + 
        [parent directory(s)]<parent permission flag + information>
    -h | --help
        Prints this text
");
}

/**
 * Perform operations to create a directory 
 * 
 * Accepts arguments use -h or --help for full list
 */
fn main() {

    let args: Vec<String> = env::args().collect(); 
    let mut permission_level: u32 = 493;
    let mut itr = 0;
    let mut directory_name: String;

    if args.len() <= 1 {
        print_help_menu();
        return;
    } else {
        directory_name = args[1].clone();
    }    
    while itr < args.len() {
        match args[itr].as_str() {
            "-p" | "--permissions" => {
                if itr + 1 < args.len() {

                    let permission_arg = &args[itr + 1];
                    if let Ok(plvl) = u32::from_str_radix(permission_arg, 8) {
                        permission_level = plvl;
                    } else {
                        eprintln!("ERR: Invalid value for permissions: {}", permission_arg);
                        return;
                    }
                } else {
                    eprintln!("ERR: No value provided for permissions after {}", args[itr]);
                    return;
                }
            },
            "-P" | "--parents" => {
                if itr + 1 >= args.len() {
                    eprintln!("ERR: flag given but no arguments provided!");
                    return;
                }

                let mut parent_permissions: Vec<u32> = Vec::new();
                let mut parent_names: Vec<String> = Vec::new();

                let mut itr2 = itr + 1;

                // Begin iteration over remainder of args (second round iteration)
                let mut before_pp = true;
                while itr2 < args.len() {
                    match args[itr2].as_str() {
                        "-pp" | "--parent_permissions" => {
                            before_pp = false;
                            if itr2 + 1 >= args.len() {
                                eprintln!("ERR: No value provided for parent permissions after {}", args[itr2]);
                                return;
                            }

                        }
                        _ => {
                            if before_pp == true{
                                parent_names.push(args[itr2].clone());
                            } else {
                                let parg = &args[itr2];
                                    
                                if let Ok(plvl) = u32::from_str_radix(&parg, 8) {
                                    println!("{:o}", plvl);
                                    parent_permissions.push(plvl);
                                } else {
                                    eprintln!("ERR: Invalid value for parent permissions: {}", parg);
                                    return;  
                                }
                            }
                            println!("Passed here")
                        }
                    }

                    itr2 += 1; 
                }
                // End second round iteration

                /*for name in parent_names.clone() {
                    println!("DIR: {}", name);
                }
                println!("Names len: {}", parent_names.len());

                for value in parent_permissions.clone() {
                    println!("PERM: {:o}", value);
                }
                println!("Perms len: {}", parent_permissions.len());

                if parent_permissions.len() != parent_names.len() {
                    eprintln!("ERR: parent permissions and parent directories given but some directories unpermissioned!");
                    return;
                }*/

                let mut parentitr = 0;
                let mut super_path: String = "".to_string();

                while parentitr < parent_names.len() {

                    if parent_permissions.len() != 0 {

                        super_path.push_str(parent_names[parentitr].as_str());
                        create_directory(&super_path.as_str(),parent_permissions[parentitr]);
                        super_path.push_str("/");

                    } else {

                        super_path.push_str(parent_names[parentitr].as_str());
                        create_directory(&super_path.as_str(), 493);
                        super_path.push_str("/");

                    } 
                    parentitr += 1; 
                }

                super_path.push_str(directory_name.as_str());
                directory_name = super_path;
            },

            "-h" | "--help" => {
                print_help_menu();
                return;
            },
            _ => {
                print!("")
            }
        }

        itr += 1
    }

    create_directory(&directory_name, permission_level);

}
