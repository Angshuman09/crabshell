use std::env;
use std::path::Path;
use std::fs;
use std::os::unix::fs::PermissionsExt;

pub fn find_in_path(cmd: &str) -> Option<String>{
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