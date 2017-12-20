use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufRead;
use std::fs;
use std::fs::File;
// use custom libs
use conf;

/// This first checks if the log file exist
/// if not, it will create it. For now it wil create
/// the file in /home/dakota/clear_cache_logs .
// Ideally it should be under /var
pub fn create_log_file(debug: bool){
	let log_path = "/home/dakota/clear_cache_logs";

	if conf::check_if_path_exist(log_path) == true{
		if debug == true {
			println!("Log file already exist");
		}
	}else {
		let mut log_file = File::create(log_path).expect("Unable to create file");
		log_file.write_all(b"File Created").unwrap();
		log_file.sync_all().unwrap();
	}	
}

/// This function delete users cache
pub fn delete_user_cache(debug: bool){
	//[user_file]
	//[user_dir]
	//[system_file]
	//[system_dir]
	let sec = "[user_file]{".to_string();
	let path_dir_vec = conf::parse_config(&sec, debug);

	for x in &path_dir_vec{
		if conf::check_if_path_exist(x) == true{
			fs::remove_dir_all(x).expect("Failded to delete");
			if debug == true {
				println!("Deleted Dir: {}", x);
			}
		} else {
			if debug == true {
				println!("Dir didn't exist: {}", x);
			}	
		}
	}

	// this used for deleting invidual files
	let sec1 = "[user_dir]{".to_string();
	let path_file_vec = conf::parse_config(&sec1, debug);

	for x in &path_file_vec{
		if conf::check_if_path_exist(x) == true{
			fs::remove_file(x).expect("Failded to delete");
			if debug == true {
				println!("Deleted File: {}", x);
			}
		} else {
			if debug == true{
				println!("File didn't exist: {}", x);
			}
		}
	}
}

// This function delete users cache copy delete_user_cache
//pub fn delete_system_cache(){

//}
