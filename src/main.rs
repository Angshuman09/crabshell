use std::io::{self, Write};
use std::path::{ Path};
use std::env;

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
            }else if let Some(path) = find_in_path(cmd){
                println!("{} is {}",cmd_to_check, path);
            }else{
                println!("{}: not found", cmd_to_check);
            }
        },
        _ =>{
            println!("{} not found", cmd);
        }
    }
    }
}

fn find_in_path(cmd: &str) -> Option<String>{
    let path_env = env::var("PATH").ok()?;

    for dir in path_env.split(":"){
        let full_path = Path::new(dir).join(cmd);

        if full_path.is_file(){
            return Some(full_path.to_string_lossy().to_string());
        }
    }
    None
}