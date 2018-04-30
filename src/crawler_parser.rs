/*
This software is copyrighted under the Apache 2 License
in the year of our lord and savior christ.
Contact: bearzrobotics@gmail.com

You should have received a copy of the Apache 2 License
along with this program. .

live honorably, harm no one, give to each his own.
*/

//use std::fs::File;
//use std::io::BufReader;
//use std::process;
use std::fs;
use utils;
use conf_parser;
use cleaner;

pub fn get_crawler_files(mode: u8, crawler_dir: String) -> Vec<String>{
    let mut crawler_files = Vec::new();
    let paths = fs::read_dir(&crawler_dir).unwrap();

    for path in paths {
        let pth = path.unwrap().path().file_name().unwrap().to_string_lossy().into_owned();
        let mut crawler_path = crawler_dir.clone();
        crawler_path.push_str(&pth);

        if crawler_path != " ".to_string(){
            crawler_files.push(crawler_path);
        }
    }
            
    if mode == 1 {
        for i in 0..crawler_files.len(){
            println!("{:?}", crawler_files[i]);
        }
    }
    crawler_files
}

pub fn crawler_interater(crawler_files: Vec<String>, mode: u8, delete_file: u8){ 
    for i in 0..crawler_files.len(){
        let element_vec = conf_parser::read_file(&crawler_files[i].to_string(), mode, 2);
        println!(" ");
        element_parser(mode, element_vec, delete_file);
    }

}

pub fn element_parser(mode: u8, elements: Vec<String>, delete_file: u8){
    
    for i in 0..elements.len(){
        // as of right now we don't info

        if elements[i].starts_with("name=") == true{
            let mut tmp = get_data(elements[i].to_string());
        }
        if elements[i] == "root=yes" && utils::am_root() == true{
            element_parser(mode, elements.clone(), delete_file);
        }else{
            error!("not running as root");
        }

        if cfg!(target_family = "unix"){

            if elements[i].starts_with("find=") == true{
                println!("find");
            }

            if elements[i].starts_with("delete=") == true && delete_file == 1{
                let tmp_path = get_data(elements[i].to_string());
                let path = utils::get_env(tmp_path);
                cleaner::single_delete(mode, path);
            }
        }else{
            if elements[i].starts_with("wfind=") == true{
                println!("wfind");
            }  

            if elements[i].starts_with("delete=") == true && delete_file == 1{
                let tmp_path = get_data(elements[i].to_string());
                let path = utils::get_env(tmp_path);
                cleaner::single_delete(mode, path);
            }

        }
    }
}

/// This funcion takes in an element from the config and returns the actual path data
fn get_data(data: String) -> String {
    let index = data.find('=').unwrap();
    let new_str = &data[&index+1..];   
    new_str.to_string()
}

