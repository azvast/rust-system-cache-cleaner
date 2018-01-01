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
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::env;



// https://askubuntu.com/questions/410244/a-command-to-list-all-users-and-how-to-add-delete-modify-users
// This command works awk -F'[/:]' '{if ($3 >= 1000 && $3 != 65534) print $1}' /etc/passwd
// https://stackoverflow.com/questions/33294932/parsing-variable-from-delimited-file
// Because of my uncle's hackery we will not by filtering out below 65543

/// this will filter the vec parsed by the filter_password
/// There is a method to the madness. This program is ment to run 
/// on my uncle johns system (Slackware) as well as my own (Solus), 
/// which he creates users in non convensional places such as /x or /y and
/// gives them user id's less than 1000. So this parse out user paths with / or /dev/null all
/// others it will test with the check_path functions.
pub fn get_users(mode: u8) -> Vec<String>{

    let mut user_path: Vec<String> = Vec::new();                // used to return the path

    if am_root() == true {
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

    }else{
        let home = env::var("HOME").expect("Couldn't read Var");
        user_path.push(home);

        if mode == 1 {
            println!("User_Path: {}", user_path[0]);
        }
        return user_path
    }
}

pub fn get_log_path(mode: u8) -> String{
	let mut home: String = env::var("HOME").expect("Couldn't find env HOME");
    
    if home == "/root" {
        home = "/var/cache_cleaner".to_string();
    }

    let log_path: &str = "/.cache_cleaner_logs";

    home.push_str(log_path);

    if mode == 1 {
        println!("Log Path: {}", &home);
        return home
    } else {
        return home
    }
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

pub fn am_root() -> bool {
    match env::var("USER") {
        Ok(val) => val == "root",
        Err(_e) => false,
    }
}

