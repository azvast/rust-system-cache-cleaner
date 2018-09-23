/*
This software is copyrighted under the Apache 2 License
in the year of our lord and savior christ.
Contact: bearzrobotics@gmail.com

You should have received a copy of the Apache 2 License
along with this program. .

live honorably, harm no one, give to each his own.
*/
use wild;
use std::{env, fs};
use users;
use crossterm::crossterm_style::{paint, Color};

pub fn find(mode: u8, path: String) -> String{
    if check_if_path_exist(&path) == true{

    }else{
        println!("{}: {}", paint("File/Dir didn't exist").with(Color::Red), path);
    }

    path
}

/// This function checks if a file exist 
pub fn check_if_path_exist(path: &String) -> bool{
	fs::metadata(path).is_ok()
}

/// 0 - means the path is nether a file or dir
/// 1 - means its a dir
/// 2 - means its a file
pub fn check_if_file(path: &String) -> u8{
    let work_path = fs::metadata(path).expect("Unable to open");
    
    if work_path.is_dir() == true{
        return 1
    } else if work_path.is_file() == true{
        return 2
    }else{
        return 0
    }
}



// https://askubuntu.com/questions/410244/a-command-to-list-all-users-and-how-to-add-delete-modify-users
// This command works awk -F'[/:]' '{if ($3 >= 1000 && $3 != 65534) print $1}' /etc/passwd
// https://stackoverflow.com/questions/33294932/parsing-variable-from-delimited-file
// Because of my uncle's hackery we will not by filtering out below 65543

/// this will filter the vec parsed by the filter_password
/// There is a method to the madness. This program is ment to run 
/// on my uncle johns system (Slackware) as well as my own (Solus), 
/// which he creates users in non convensional places such as /x or /y and
/// gives them user id's less than 1000. So this parse out user paths with / or /dev/null all
/// others it will test with the check_path functions.
//#[cfg(target_os = "linux")]
pub fn get_users(mode: u8) -> Vec<String>{
    let mut user_path: Vec<String> = Vec::new();                // used to return the path

    if am_root() == true {
        user_path = users::users(mode);
        return user_path
        
    }else{
        let home = {
            if cfg!(windows){
                env::var("USERPROFILE").expect("Couldn't read Var USERPROFILE")
            } else {
                env::var("HOME").expect("Couldn't read Var HOME") 
            }
        };
        user_path.push(home);

        if mode == 1 {
            println!("User_Path: {}", user_path[0]);
        }
        return user_path
    }
}


pub fn get_log_path(mode: u8) -> String{
    let mut home: String = {
        if cfg!(windows){
            env::var("USERPROFILE").expect("Couldn't find env USERPROFILE")
        }else{
            env::var("HOME").expect("Couldn't find env HOME")
        }
    }; 
	
    let log_path: &str = "/.cache_cleaner_logs";

    if home == "/root" {
        home = "/var/cache_cleaner/cache_cleaner_logs".to_string();
    }else{
        home.push_str(log_path);
    }

    if mode == 1 {
        println!("Log Path: {}", &home);
        return home
    } else {
        return home
    }
}

/// This will try and create a fill in a system dir
/// If it was able to create the file you have admin
/// access. If not you don't.
#[cfg(target_os = "windows")] // for windows 
pub fn am_root() -> bool {
    let pth = String::from("WINDIR\\cache_cleaner");

    fs::create_dir(&pth);

    if check_if_path_exist(&pth) == true{
        fs::remove_file(&pth).expect("Failded to delete");
        return true
    }else{
        return false
    }
}

#[cfg(any(unix))]
pub fn am_root() -> bool {
    match env::var("USER") {
        Ok(val) => val == "root",
        Err(_e) => false,
    }
}


pub fn embed_env_into_string(env_name: String, path: String) -> String{
    let var = env::var(env_name).expect("Couldn't read envermont varibale");
    let mut tmp_path: String = var.to_string();
    tmp_path.push_str(&path);
    tmp_path
}

pub fn get_env(path: String) -> String{
    let index ={
        if cfg!(windows){
            path.find('\\').unwrap()
        }else{
            path.find('/').unwrap()
        }
    };

    let tmp_path = &path[index..];      // gets the rest of the path
    let env_name =&path[..index];       //gets the env name at front of path
    
    let out_path = {
        if index <= 0{
            path.to_string()
        }else{ // This is here in case the being of the string isn't an env
            embed_env_into_string(env_name.to_string(), tmp_path.to_string())
        }
    }; 

    out_path
}