mod parser;
mod utils;
mod builtins;
use std::io::{self, Write};
use crate::builtins::{ handle_cd, handle_echo, handle_pwd, handle_type};
use crate::utils::find_in_path;
use std::process::Command;
use crate::parser::tokenize;

fn main(){
    loop{
    print!("$ ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    if input.is_empty() { continue; }

    let token = tokenize(input);
    let command = &token[0];
    let args = &token[1..];

    match command.as_str(){

        "exit"=> break,
        "echo"=> handle_echo(args),
        "pwd"=> handle_pwd(),
        "cd"=>handle_cd(args),
        "type"=> handle_type(args),
        external_cmd=>{
            if let Some(path) = find_in_path(external_cmd){
                let mut child = Command::new(path)
                                    .args(args)
                                   .spawn()
                                    .expect("Failed to execute command");
                                // println!("{:?}", child );

                                child.wait().expect("Process wasn't running");
            }else{
                println!("{} command not found", external_cmd);
            }
        }
    }
    }
}

