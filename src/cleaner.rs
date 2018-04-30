/*
This software is copyrighted under the Apache 2 License
in the year of our lord and savior christ.
Contact: bearzrobotics@gmail.com

You should have received a copy of the Apache 2 License
along with this program. .

live honorably, harm no one, give to each his own.
*/
use std::fs;
use crossterm::crossterm_style::{paint, Color};
// use custom libs
use conf_parser;
use utils;

/// This first checks if the log file exist
/// if not, it will create it. For now it wil create
/// the file in /home/dakota/clear_cache_logs .
// Ideally it should be under /var

/// This function delete users cache
pub fn delete_user_cache(mode: u8, config_path: &String){
	//[user_dir]{
	//[user_file]{
	let sec = "[user_file]{".to_string();
	let sec1 = "[user_dir]{".to_string();

	section(mode, &sec1, &config_path);
	section(mode ,&sec, &config_path);
}

// This function delete system cache 
pub fn delete_system_cache(mode: u8, config_path: &String){
	//[system_file]{
	//[system_dir]{
	let sec = "[system_file]{".to_string();
	let sec1 = "[system_dir]{".to_string();
	
	if utils::am_root() == true{	
		info!("Running as root");
		section(mode, &sec1, &config_path);
		section(mode, &sec, &config_path);
	}else {
		error!("Not running as root");
	}
}


fn section(mode: u8, sec: &String, config_path: &String){
	let home: Vec<String>  = utils::get_users(mode);
	let tmp_path_vec = conf_parser::parse_config(&sec, mode, &config_path);
	let mut path_vec = Vec::new();

	if sec == "[user_file]{" || sec == "[system_file]{"{
		for i in 0..home.len(){
			for x in 0..tmp_path_vec.len(){
				path_vec.push(home[i].to_string() + &tmp_path_vec[x].to_string());
				if mode == 1 {
					println!("{}", path_vec[x]);
				}
			}
		}
	}
	if sec == "[user_dir]{" || sec == "[system_dir]{"{
		for i in 0..home.len(){
			for x in 0..tmp_path_vec.len(){
				path_vec.push(home[i].to_string() + &tmp_path_vec[x].to_string());
				if mode == 1 {
					let vec_slice: &str = &path_vec[x];
					println!("vec_slice[{}]: {}", x, vec_slice);
				}
			}
		}
	}
	delete(mode, path_vec);
}

/// This fuction takes a vector of paths to parse through and delete
pub fn delete(mode: u8, paths: Vec<String>){
	for i in 0..paths.len(){
		if utils::check_if_path_exist(&paths[i]) == true{
			if utils::check_if_file(&paths[i]) == 1 {
				fs::remove_dir_all(&paths[i]).expect("Failded to delete dir");
				if (mode == 2) || (mode == 1){
					println!("{}: {}", paint("Deleted dir").with(Color::Green), paths[i]);
				}
			}else if utils::check_if_file(&paths[i]) == 2{
					fs::remove_file(&paths[i]).expect("Failded to delete file");
				if (mode == 2) || (mode == 1){
					println!("{}: {}", paint("Deleted file").with(Color::Green), paths[i]);
				}		
			}else{
				println!("This is not a file or dir: {}", paths[i]);
			} 	
		}else{
			if (mode == 2) || (mode == 1){
				println!("{}: {}", paint("File/Dir didn't exist").with(Color::Red), paths[i]);
			}
		} 
	}
}

/// this function is used to delete one file at a time.
pub fn single_delete(mode: u8, path: String){
	if utils::check_if_path_exist(&path) == true{
		if utils::check_if_file(&path) == 1 {
			fs::remove_dir_all(&path).expect("Failded to delete dir");
			if (mode == 2) || (mode == 1){
				println!("{}: {}", paint("Deleted dir").with(Color::Green), path);
			}
		}else if utils::check_if_file(&path) == 2{
				fs::remove_file(&path).expect("Failded to delete file");
			if (mode == 2) || (mode == 1){
				println!("{}: {}", paint("Deleted file").with(Color::Green), path);
			}		
		}else{
			println!("This is not a file or dir: {}", path);
		} 	
	}else{
		if (mode == 2) || (mode == 1){
			println!("{}: {}", paint("File/Dir didn't exist").with(Color::Red), path);
		}
	} 

}