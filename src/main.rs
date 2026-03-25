use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::env;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;

fn main(){
    let builtins = ["echo", "exit", "type"];

    loop{
    print!("$ ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    if input.is_empty() { continue; }

    let parts: Vec<&str> = input.split_whitespace().collect();

    let cmd = parts[0];
    let args = &parts[1..];

    match cmd{

        "exit" => break,
        "echo" => println!("{}", args.join(" ")),
        "type" =>{
            if args.is_empty() { continue; }

            let cmd_to_check = args[0];
            
            if builtins.contains(&cmd_to_check){
                println!("{} is a shell builtin", cmd_to_check);
            }else if let Some(path) = find_in_path(cmd_to_check){
                println!("{} is {}",cmd_to_check, path);
            }else{
                println!("{}: not found", cmd_to_check);
            }
        },
        _ =>{
            if let Some(path) = find_in_path(cmd){
                let mut child = Command::new(path)
                                    .args(args)
                                    .spawn()
                                    .expect("Failed to execute command");
                                // println!("{:?}", child );

                                child.wait().expect("Process wasn't running");
            }else{
                println!("{} command not found", cmd);
            }
        }
    }
    }
}

fn find_in_path(cmd: &str) -> Option<String>{
    let path_env = env::var("PATH").ok()?;
    // println!("path env: {:?}", path_env );

    for dir in path_env.split(":"){
        let full_path = Path::new(dir).join(cmd);
        // println!("full path: {:?}", full_path );

        if let Ok(metadata) = fs::metadata(&full_path){
            // println!("{:?}", metadata);
            if metadata.is_file(){
                let mode = metadata.permissions().mode();
                // println!("{}",mode);
                if mode & 0o111 != 0{
                    return Some(full_path.to_string_lossy().to_string());
                }
            }
        }

    }
    None
}