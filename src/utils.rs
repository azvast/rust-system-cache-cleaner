use std::io::{BufReader, BufRead};
use std::fs::File;
use std::env;
//https://askubuntu.com/questions/410244/a-command-to-list-all-users-and-how-to-add-delete-modify-users
// This command works awk -F'[/:]' '{if ($3 >= 1000 && $3 != 65534) print $1}' /etc/passwd
// https://stackoverflow.com/questions/33294932/parsing-variable-from-delimited-file
// Because of my uncle's hackery we will not by filtering out below 65543
fn _get_users(){

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
    let mut user_name = String::new();      // used for returning the user name
    let mut user_path = String::new();      // used to return the path

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
        Err(e) => false,
    }
}