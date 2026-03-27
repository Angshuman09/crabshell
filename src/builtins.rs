use std::env;
use std::path::Path;
use crate::utils::find_in_path;

pub const BUILTINS: [&str; 5] = ["echo", "exit", "pwd", "type", "cd"];

pub fn is_builtins(cmd: &str) -> bool{
    BUILTINS.contains(&cmd)
}

pub fn handle_echo(args: &[String]) {
    println!("{}", args.join(" "));
}

pub fn handle_pwd() {
    if let Ok(current_dir) = env::current_dir() {
        println!("{:?}", current_dir.display());
    } else {
        eprintln!("pwd: couldn't get the current directory.");
    }
}

pub fn handle_cd(args: &[String]) {
    if args.is_empty() {
        return;
    }
    let mut path_str = args[0].to_string();
    // println!("path str: {}", path_str);

    if path_str == "~" {
        if let Ok(home) = env::var("HOME") {
            path_str = home;
        }
    }

    let new_path = Path::new(&path_str);
    // println!("new path: {:?}", new_path);
    if let Err(_) = env::set_current_dir(new_path) {
        println!("cd: {}: No such file or directory exist", path_str);
    }
}

pub fn handle_type(args: &[String]){
    if let Some(cmd) = args.get(0){
        if is_builtins(cmd){
            println!("{} is a shell builtin", cmd);
        }
        else if let Some(path) = find_in_path(cmd){
            println!("{} is {}", cmd, path);
        }else{
            println!("{} not found", cmd);
        }
    }
}