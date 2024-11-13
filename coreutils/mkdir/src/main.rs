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
        println!("
Create a directory if it does not already exist.
Input format: [directory name]<flags + information>
Flags:
    -p | --permissions
        Provide flag and permission set in octal
    -P | --parents 
        Provide a list of parent directories. If they need to have special permissions, 
        use -pp or --parent-permissions.
        This modifies the input format to [directory name]<flags + information> + 
        [parent directory(s)]<parent permission flag + information>
    -h | --help
        Prints this text");
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

                while itr2 < args.len() {
                    match args[itr2].as_str() {
                        "-pp" | "--parent_permissions" => {

                            if itr2 + 1 >= args.len() {
                                eprintln!("ERR: No value provided for parent permissions after {}", args[itr2]);
                                return;
                            } else {
                                while itr2 + 1 < args.len() {

                                    let parg = &args[itr2 + 1];
    
                                    if let Ok(plvl) = u32::from_str_radix(parg, 8) {
                                        parent_permissions.push(plvl);
                                    } else {
                                        eprintln!("ERR: Invalid value for parent permissions: {}", parg);
                                        return;
                                    }
    
                                    itr2 += 1; 
                                }
                                itr2 = args.len();
                            }

                        }
                        _ => {
                            parent_names.push(args[itr2].clone());
                        }
                    }

                    itr2 += 1; 
                }

                let mut parentitr = 0;
                let mut super_path: String = "".to_string();

                while parentitr < parent_names.len() {

                    if parentitr > 0 && parent_permissions.len() != 0{

                        if parentitr == 0 && parent_permissions.len() != 0 {

                            if parent_permissions.len() != parent_names.len() {
                                eprintln!("ERR: parent permissions and parent directories given but some directories unpermissioned!");
                                return;
                            }
    
                            super_path.push_str(parent_names[parentitr].as_str());
                            create_directory(&super_path.as_str(), parent_permissions[0]);
                            super_path.push_str("/");
    
                        }

                        super_path.push_str(parent_names[parentitr].as_str());
                        create_directory(&super_path.as_str(),parent_permissions[parentitr]);
                        super_path.push_str("/");

                    } else if parentitr == 0 && parent_permissions.len() == 0 {

                        super_path.push_str(parent_names[parentitr].as_str());
                        create_directory(&super_path.as_str(), 493);
                        super_path.push_str("/");

                    } else if parentitr > 0 && parent_permissions.len() == 0 {

                        super_path.push_str(parent_names[parentitr].as_str());
                        create_directory(&super_path.as_str(), 493);
                        super_path.push_str("/");

                    } 

                    if parent_names[parentitr+1] == "-PP" || parent_names[parentitr+1] == "--parent-permissions" {parentitr = parent_names.len() + 1} else {parentitr += 1;}
                }

                super_path.push_str(directory_name.as_str());
                directory_name = super_path;
            },

            "-h" | "--help" => {
                println!("
Create a directory if it does not already exist.
Input format: [directory name]<flags + information>
Flags:
    -p | --permissions
        Provide flag and permission set in octal
    -P | --parents 
        Provide a list of parent directories. If they need to have special permissions, 
        use -pp or --parent-permissions.
        This modifies the input format to [directory name]<flags + information> + 
        [parent directory(s)]<parent permission flag + information>
    -h | --help
        Prints this text");
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
