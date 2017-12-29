use std::fs;
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

/// Note I believe the for loops arn't running because the vector isn't returning a proper lenght
fn delete_dir(mode: u8, sec: &String){

	let home: Vec<String>  = utils::get_users(mode);
	let tmp_path_dir_vec = conf::parse_config(&sec, mode);
	let mut path_dir_vec = Vec::new();

	if sec == "[user_dir]{"{
		for i in 0..home.len(){
			for x in 0..tmp_path_dir_vec.len(){
				path_dir_vec.push(home[i].to_string() + &tmp_path_dir_vec[x].to_string());
				if mode == 1 {
					println!("{}", path_dir_vec[x]);
				}
			}
		}
	}

	for x in 0..path_dir_vec.len(){
		if conf::check_if_path_exist(&path_dir_vec[x]) == true{
			fs::remove_dir_all(&path_dir_vec[x]).expect("Failded to delete");
			if (mode == 2) || (mode == 1){
				println!("Deleted dir: {}", path_dir_vec[x]);
			}
		} else {
			if (mode == 2) || (mode == 1){
				println!("Dir didn't exist: {}", path_dir_vec[x]);
			}
		}
	}
	// Have value here
}

fn delete_file(mode: u8, sec: &String){
	let home: Vec<String>  = utils::get_users(mode);
	let tmp_path_file_vec = conf::parse_config(&sec, mode);
	let mut path_file_vec = Vec::new();

	if sec == "[user_file]{" {
		for i in 0..home.len(){
			for x in 0..tmp_path_file_vec.len(){
				path_file_vec.push(home[i].to_string() + &tmp_path_file_vec[x].to_string());
				if mode == 1 {
					println!("{}", path_file_vec[x]);
				}
			}
		}
	}

	for x in 0..path_file_vec.len(){
		// I believe this isn't working at all
		if conf::check_if_path_exist(&path_file_vec[x]) == true{
			fs::remove_file(&path_file_vec[x]).expect("Failded to delete");
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

