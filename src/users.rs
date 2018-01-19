use std::io::{BufReader, BufRead};
use std::fs::File;
use utils;

pub fn linux_users(mode: u8) -> Vec<String>{
    let mut user_path: Vec<String> = Vec::new(); 
    let (user_vec, line_counter) = filter_passwd(mode);
    let mut index = 5;                                          // This is the sixths value of the passwd file

    let kill_index = line_counter * 7;

    for i in 0..user_vec.len()/7{
        index  = index + 7;
        if index >= kill_index {
            break;
        }
        if mode == 1 {
            println!("i: {}", i);
        }

        if user_vec[index] != "/" && user_vec[index] != "/dev/null" && user_vec[index] != "/var/lib/avahi-autoipd" && user_vec[index] != "/var/spool/cups" && user_vec[index] != "/var/lightdm" && user_vec[index] != "/var/lib/colord" && user_vec[index] != "/var/run/dbus"{
            let tmp = &user_vec[index];
            user_path.push(tmp.to_string());

            if mode == 1 {
                println!("User_Vec: {}", &user_vec[index]);
            }
        }
    }

    if mode == 1 {            // this is hear to make sure its building the new vector right. Which it does so far.
        for i in 0..user_path.len(){
            println!("User_Path: {}", user_path[i]);
        }
    }
    return user_path
}

fn filter_passwd(mode: u8) -> (Vec<String>, usize){
    //  0    1   2    3     4       5           6
    //dakota:x:1000:1000:dakota:/home/dakota:/bin/bash
    // Example of what file should look like.
    // There are 7 (0-6) fields to parse
    // we need 0 and 5
    let file_in = File::open("/etc/passwd").expect("No such file");

    let reader = BufReader::new(file_in);

    let mut count = 0;                               // used for counting in the for looops
    let mut line_counter: usize = 0;                 // This keeps track of how man lines we interate through
    let mut pass_vec: Vec<String> = Vec::new();      // creates a vector for storing values in it.
                                                     // not calling it work_vec for a change because i actually have a better name.
    for line in reader.lines(){
        line_counter = line_counter + 1;

        for value in line.unwrap().split(':') {
            if mode == 1{
                println!("{}", &value);
            }
            pass_vec.push(value.to_string());
            count = count + 1;
        }
        // This is here for debug information line_counter * 7 should give you the total index value.
        if mode == 1{
            println!("Line Counter: {}", &line_counter);
        }
    }
    (pass_vec, line_counter)
}