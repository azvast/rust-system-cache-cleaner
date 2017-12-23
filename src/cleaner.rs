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
	let log_path = "/home/dakota/clear_cache_logs".to_string();

	if conf::check_if_path_exist(&log_path) == true{
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
	delete_user_file(debug);
	println!("============================");
	println!("");
	println!("============================");
	delete_user_dir(debug);
}

fn delete_user_dir(debug: bool){
	//[user_dir]
	// this used for deleting invidual files
	let sec1 = "[user_dir]{".to_string();
	let (path_file_vec, start_line, end_line) = conf::parse_config(&sec1, debug);

	for x in start_line..path_file_vec.len(){
		let for_vec = path_file_vec[x].to_string();

		if x == end_line{
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

fn delete_user_file(debug: bool){
	//[user_file]
	let sec = "[user_file]{".to_string();
	let (path_dir_vec, start_line, end_line) = conf::parse_config(&sec, debug);

	for x in start_line..path_dir_vec.len(){
		let for_vec = path_dir_vec[x].to_string();

		if x == end_line{
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



// This function delete system cache 
pub fn delete_system_cache(debug: bool){
	//[system_file]{
	//[system_dir]{
	let sec = "[system_file]{".to_string();
	let (path_dir_vec, start_line, end_line) = conf::parse_config(&sec, debug);

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
	let sec1 = "[system_dir]{".to_string();
	let (path_file_vec, start_line, end_line) = conf::parse_config(&sec1, debug);

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
