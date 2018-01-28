/*
KNOW ALL MEN BY THESE PRESENTS: 'i': man [Dakota James Owen Keeler]
copyright this software in the year of our lord 2017 under the Apache 2
Contact: bearzrobotics@gmail.com

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

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

	delete_dir(mode, &sec1, &config_path);
	delete_file(mode ,&sec, &config_path);
}

// This function delete system cache 
pub fn delete_system_cache(mode: u8, config_path: &String){
	//[system_file]{
	//[system_dir]{
	let sec = "[system_file]{".to_string();
	let sec1 = "[system_dir]{".to_string();
	
	if utils::am_root() == true{	
		info!("Running as root");
		delete_dir(mode, &sec1, &config_path);
		delete_file(mode, &sec, &config_path);
	}else {
		error!("Not running as root");
	}
}

fn delete_dir(mode: u8, sec: &String, config_path: &String){

	let home: Vec<String>  = utils::get_users(mode);
	let tmp_path_dir_vec = conf_parser::parse_config(&sec, mode, &config_path);
	let mut path_dir_vec = Vec::new();

	if sec == "[user_dir]{"{
		for i in 0..home.len(){
			for x in 0..tmp_path_dir_vec.len(){
				path_dir_vec.push(home[i].to_string() + &tmp_path_dir_vec[x].to_string());
				if mode == 1 {
					let vec_slice: &str = &path_dir_vec[x];
					println!("vec_slice[{}]: {}", x, vec_slice);
				}
			}
		}
	}

	for x in 0..path_dir_vec.len(){
		if utils::check_if_path_exist(&path_dir_vec[x]) == true{
			fs::remove_dir_all(&path_dir_vec[x]).expect("Failded to delete");
			if (mode == 2) || (mode == 1){
				println!("{}: {}", paint("Deleted dir").with(Color::Green), path_dir_vec[x]);
			}
		} else {
			if (mode == 2) || (mode == 1){
				println!("{}: {}", paint("Dir didn't exist").with(Color::Red), path_dir_vec[x]);
			}
		}
	}
}

fn delete_file(mode: u8, sec: &String, config_path: &String){
	let home: Vec<String>  = utils::get_users(mode);
	let tmp_path_file_vec = conf_parser::parse_config(&sec, mode, &config_path);
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
		if utils::check_if_path_exist(&path_file_vec[x]) == true{
			fs::remove_file(&path_file_vec[x]).expect("Failded to delete");
			if (mode == 2) || (mode == 1){
				println!("{}: {}", paint("Deleted file").with(Color::Green), path_file_vec[x]);
			}
		} else {
			if (mode == 2) || (mode == 1){
					println!("{}: {}", paint("File didn't exist").with(Color::Red), path_file_vec[x]);
			}
		}
	}
}

