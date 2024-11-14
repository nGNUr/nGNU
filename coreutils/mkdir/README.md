# mkdir
A simple project meant to replicate the functionality of the mkdir program from GNU coreutils in rust

## Command and Flag List
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
