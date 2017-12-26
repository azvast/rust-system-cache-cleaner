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
pub fn delete_user_cache(mode: u8){
	//[user_dir]{
	//[user_file]{
	
	let sec = "[user_file]{".to_string();
	let sec1 = "[user_dir]{".to_string();

	delete_dir(mode, &sec1);
	delete_file(mode ,&sec);
	
}

// This function delete system cache 
pub fn delete_system_cache(mode: u8){
	//[system_file]{
	//[system_dir]{
	let sec = "[system_file]{".to_string();
	let sec1 = "[system_dir]{".to_string();
	
	delete_dir(mode, &sec1);
	delete_file(mode, &sec);
	
}

fn delete_dir(mode: u8, sec: &String){

	let home: Vec<String>  = utils::get_users(mode);
	let (tmp_path_file_vec, start_line, end_line) = conf::parse_config(&sec, mode);
	let mut path_file_vec = Vec::new();

	for i in 0..home.len(){
		for x in 0..path_file_vec.len(){
			path_file_vec.push(home[i].to_string() + &tmp_path_file_vec[x].to_string());
			println!("{}", path_file_vec[x]);
		}
	}
	// I have mode value here

	let tmp_mode = mode;
	for x in start_line..path_file_vec.len(){
		// I do not have mode value here
		if tmp_mode == 1 && x == end_line{
			println!("endline: {}", end_line);
			break;
		}
		println!("Value of mode {}", &tmp_mode);
		if conf::check_if_path_exist(&path_file_vec[x]) == true{
			println!("Value of mode inside of if {}", &mode);
			fs::remove_file(&path_file_vec[x]).expect("Failded to delete");
			if (tmp_mode == 2) || (tmp_mode == 1){
				println!("Deleted dir: {}", path_file_vec[x]);
			}
		} else {
			//if (mode == 2) || (mode == 1){
			if (tmp_mode == 2) || (tmp_mode == 1){
				println!("Dir didn't exist: {}", path_file_vec[x]);
			}
		}
	}
}

fn delete_file(mode: u8, sec: &String){
	let home: Vec<String>  = utils::get_users(mode);
	let (tmp_path_file_vec, start_line, end_line) = conf::parse_config(&sec, mode);
	let mut path_file_vec = Vec::new();

	for i in 0..home.len(){
		for x in 0..path_file_vec.len(){
			path_file_vec.push(home[i].to_string() + &tmp_path_file_vec[x].to_string());
			println!("{}", path_file_vec[x]);
		}
	}

	for x in start_line..path_file_vec.len(){
		if mode == 1 && x == end_line{
			println!("endline: {}", end_line);
			break;
		} 
		if conf::check_if_path_exist(&path_file_vec[x]) == true{
			fs::remove_dir(&path_file_vec[x]).expect("Failded to delete");
			if (mode == 2) || (mode == 1){
				println!("Deleted file: {}", path_file_vec[x]);
			}
		} else {
			if (mode == 2) || (mode == 1){
				println!("File didn't exist: {}", path_file_vec[x]);
			}
		}
	}
}

