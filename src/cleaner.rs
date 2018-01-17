/*
KNOW ALL MEN BY THESE PRESENTS: 'i': man [Dakota James Owen Keeler]
copyright this software in the year of our lord 2017 under the GNU
Public License version 2.
Contact: bearzrobotics@gmail.com

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 2 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <http://www.gnu.org/licenses/>.

live honorably, harm no one, give to each his own.
*/
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
	
	if utils::am_root() == true{	
		info!("Running as root");
		delete_dir(mode, &sec1);
		delete_file(mode, &sec);
	}else {
		error!("Not running as root");
	}
}

fn delete_dir(mode: u8, sec: &String){

	let home: Vec<String>  = utils::get_users(mode);
	let tmp_path_dir_vec = conf::parse_config(&sec, mode);
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

