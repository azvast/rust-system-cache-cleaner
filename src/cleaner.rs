use std::io::prelude::*;
//use std::io::BufReader;
//use std::io::BufRead;
use std::fs;
use std::fs::File;
// use custom libs
use conf;
use utils;

/// This first checks if the log file exist
/// if not, it will create it. For now it wil create
/// the file in /home/dakota/clear_cache_logs .
// Ideally it should be under /var

/// This function delete users cache
pub fn delete_user_cache(debug: bool){
	utils::write_log_file(debug, "====================== User Files ======================");
	let sec = "[user_file]{".to_string();
	delete_file(debug, &sec);

	utils::write_log_file(debug, "====================== User Directories ======================");
	let sec1 = "[user_dir]{".to_string();
	delete_dir(debug, &sec1);
}

// This function delete system cache 
pub fn delete_system_cache(debug: bool){
	//[system_file]{
	//[system_dir]{
	utils::write_log_file(debug, "====================== System Files ======================");
	let sec = "[system_file]{".to_string();
	delete_file(debug, &sec);

	utils::write_log_file(debug, "====================== System Directories ======================");
	let sec1 = "[system_dir]{".to_string();
	delete_dir(debug, &sec1);
}

fn delete_dir(debug: bool, sec: &String){
	//[user_dir]{
	
	let (path_file_vec, start_line, end_line) = conf::parse_config(&sec, debug);

	for x in start_line..path_file_vec.len(){
		let for_vec = path_file_vec[x].to_string();

		if debug == true && x == end_line{
			println!("endline: {}", end_line);
			break;
		}
		if conf::check_if_path_exist(&for_vec) == true{
			fs::remove_file(&for_vec).expect("Failded to delete");
			if debug == true {
				println!("Deleted dir: {}", for_vec);
			}
		} else {
			if debug == true{
				println!("Dir didn't exist: {}", for_vec);
			}
		}
	}
}

fn delete_file(debug: bool, sec: &String){
	//[user_file]{
	
	let (path_dir_vec, start_line, end_line) = conf::parse_config(&sec, debug);

	for x in start_line..path_dir_vec.len(){
		let for_vec = path_dir_vec[x].to_string();

		if debug == true && x == end_line{
			println!("endline: {}", end_line);
			break;
		}
		if conf::check_if_path_exist(&for_vec) == true{
			fs::remove_dir_all(&for_vec).expect("Failded to delete");
			if debug == true {
				println!("Deleted file: {}", for_vec);
			}
		} else {
			if debug == true {
				println!("File didn't exist: {}", for_vec);
			}	
		}
	}
}
