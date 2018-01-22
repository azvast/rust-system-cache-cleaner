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
use std::process;
use utils;

fn read_file(filename: &String, mode: u8) -> Vec<String>{
	let f = {
		if utils::check_if_path_exist(&filename) == true{
			File::open(&filename).expect("file not found, Make sure you installed the configs") // the error
		}else{
			println!("Make sure you installed the configs. Path didn't exits: {}", &filename);
			process::exit(0)
		}
	};
	
	let file = BufReader::new(&f);
	let mut work_vec = Vec::new();

	for line in file.lines(){
		let l = line.unwrap();
		if l.starts_with("/") == true || l.starts_with("\\") == true || l.starts_with("[") == true || l.starts_with("}") || l.starts_with("A:") == true ||  l.starts_with("B:") == true || l.starts_with("C:") == true || l.starts_with("D:") == true || l.starts_with("E:") == true || l.starts_with("F:") == true || l.starts_with("G:") == true || l.starts_with("H:") == true || l.starts_with("I:") == true || l.starts_with("J:") == true || l.starts_with("K:") == true || l.starts_with("L:") == true || l.starts_with("M:") == true || l.starts_with("N:") == true || l.starts_with("O:") == true || l.starts_with("P:") == true || l.starts_with("Q:") == true || l.starts_with("R:") == true || l.starts_with("S:") == true || l.starts_with("T:") == true || l.starts_with("U:") == true || l.starts_with("V:") == true || l.starts_with("W:") == true || l.starts_with("X:") == true || l.starts_with("Y:") == true || l.starts_with("Z:") == true || l.starts_with("a:") == true ||  l.starts_with("b:") == true || l.starts_with("c:") == true || l.starts_with("d:") == true || l.starts_with("e:") == true || l.starts_with("f:") == true || l.starts_with("g:") == true || l.starts_with("h:") == true || l.starts_with("i:") == true || l.starts_with("j:") == true || l.starts_with("k:") == true || l.starts_with("l:") == true || l.starts_with("m:") == true || l.starts_with("n:") == true || l.starts_with("o:") == true || l.starts_with("p:") == true || l.starts_with("q:") == true || l.starts_with("r:") == true || l.starts_with("s:") == true || l.starts_with("t:") == true || l.starts_with("u:") == true || l.starts_with("v:") == true || l.starts_with("w:") == true || l.starts_with("x:") == true || l.starts_with("y:") == true || l.starts_with("z:") == true{
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
pub fn parse_config(section: &String, mode: u8, config_path: &String) -> Vec<String>{

	let work_vec = read_file(&config_path, mode);
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
