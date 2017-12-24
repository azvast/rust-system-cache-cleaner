use std::io::{BufReader, BufRead, Write};
use std::fs:: {File};
use std::env;
use conf;

//https://askubuntu.com/questions/410244/a-command-to-list-all-users-and-how-to-add-delete-modify-users
// This command works awk -F'[/:]' '{if ($3 >= 1000 && $3 != 65534) print $1}' /etc/passwd
// https://stackoverflow.com/questions/33294932/parsing-variable-from-delimited-file
// Because of my uncle's hackery we will not by filtering out below 65543
fn _get_users(){

}

fn get_log_path(debug: bool) -> String{
	let mut home: String = env::var("HOME").expect("Couldn't find env HOME");
    
    if home == "/root" {
        home = "/var/cache_cleaner".to_string();
    }

    let log_path: &str = "/.cache_cleaner_logs";

    home.push_str(log_path);

    if debug == true {
        println!("Log Path: {}", &home);
        return home
    } else {
        return home
    }

}

fn _filter_passwd(){
    //  0    1  2    3      4       5           6
    //dakota:x:1000:1000:dakota:/home/dakota:/bin/bash
    // Example of what file should look like.
    // There are 7 (0-6) fields to parse
    // we need 0 and 5
    let file_in = File::open("/etc/passwd").expect("No such file");

    let reader = BufReader::new(file_in);

    let mut count = 0;                      // used for counting in the for looops
    //let mut user_name = String::new();      // used for returning the user name
    //let mut user_path = String::new();      // used to return the path

    for line in reader.lines(){
        for value in line.unwrap().split(':') {
            println!("{}", value);
            count = count + 1;
        }
    }
}

pub fn am_root() -> bool {
    match env::var("USER") {
        Ok(val) => val == "Root",
        Err(_e) => false,
    }
}

/// This creates the log in the home dir of the user that runs the command
pub fn create_log_file(debug: bool){

    let log_path = get_log_path(debug);

	if conf::check_if_path_exist(&log_path) == true{
		if debug == true {
			println!("Log file already exist");
		}
	}else {
		let mut log_file = File::create(&log_path).expect("Unable to create file");
		log_file.write_all(b"File Created").unwrap();
		log_file.sync_all().unwrap();
	}	
}

pub fn write_log_file(debug: bool, message: &str){
    let log_path = get_log_path(debug);

    let mut log_file = File::create(&log_path).expect("Ubable to open file");
    log_file.write(message.as_bytes()).expect("Couldn't Right file");
    log_file.sync_all().unwrap();
}