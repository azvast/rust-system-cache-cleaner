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
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::fs;

//static pth: &'static String = "/home/dakota/git/cache_cleaner/src/config/clear_cache.conf";
//const pth: str = "/home/dakota/git/cache_cleaner/src/config/clear_cache.conf";

/// This function checks if a file exist 
pub fn check_if_path_exist(path: &String) -> bool{
	fs::metadata(path).is_ok()
}

fn read_file(filename: &String, mode: u8) -> Vec<String>{
	
	let f = File::open(&filename).expect("file not found");
	let file = BufReader::new(&f);
	let mut work_vec = Vec::new();

	// strip the comments out and vec the file
	for line in file.lines(){
		let l = line.unwrap();
		if l.starts_with("/") == true || l.starts_with("[") == true || l.starts_with("}"){
			work_vec.push(l);  
		}
	}
	if mode == 1{
		for i in 0..work_vec.len(){
			println!("Debug {}", work_vec[i].to_string());
		}
	}
	work_vec
}

// parse the user config
// configs are insdie of code blocks {}
//[user_file]{
//[user_dir]{
//[system_file]{
//[system_dir]{
pub fn parse_config(section: &String, mode: u8) -> Vec<String>{

	let pth = "/etc/cache_cleaner/cache_cleaner.conf".to_string();

	let work_vec = read_file(&pth, mode);
	let mut out_vec: Vec<String> = Vec::new();
	let sec = section.to_string();
	let mut starting_index: usize = 0;
	let mut count: usize = 0;

	// parse the file 
	for i in 0..work_vec.len(){
		count = count + 1;
		if work_vec[i] == sec{
			// The plus one is to not include the header itself
			starting_index = i + 1;
			if mode == 1 {
				println!("Starting index: {}", work_vec[starting_index - 1]);
			}
		}
	}
	//println!("Out side for loop staring index {}", starting_index);

	// get end line
	for i in starting_index..work_vec.len(){
		let temp = &work_vec[i];
		out_vec.push(temp.to_string());
		if work_vec[i] == "}"{
			out_vec.pop();
			break;
		}
	}

	if mode == 1{
		for i in 0..out_vec.len() {
			println!("Out Vec: {}", out_vec[i]);
		}		
		println!(" ");
	}
	out_vec
}