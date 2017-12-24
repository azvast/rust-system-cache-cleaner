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

fn read_file(filename: &String, debug: bool) -> Vec<String>{
	
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
	if debug == true{
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
pub fn parse_config(section: &String, debug: bool) -> (Vec<String>, usize, usize){

	let pth = "/etc/clear_cache/clear_cache.conf".to_string();

	let work_vec = read_file(&pth, debug);
	let sec = section.to_string();
	let mut starting_index: usize = 0;
	let mut count: usize = 0;
	let mut end_index: usize = 0;

	// parse the file 
	for i in 0..work_vec.len(){
		count = count + 1;
		if work_vec[i] == sec{
			// The plus one is to not include the header itself
			starting_index = i + 1;
			if debug == true {
				println!("Starting index: {}", work_vec[starting_index - 1]);
			}
		}
	}
	//println!("Out side for loop staring index {}", starting_index);

	// get end line
	for i in starting_index..work_vec.len(){
		if work_vec[i] == "}"{
			end_index = i - 1;
			break;
		}
	}
	return (work_vec, starting_index, end_index)
}