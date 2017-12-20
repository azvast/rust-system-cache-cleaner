use std::io::{BufReader, BufRead};
use std::fs::File;
//https://askubuntu.com/questions/410244/a-command-to-list-all-users-and-how-to-add-delete-modify-users
// This command works awk -F'[/:]' '{if ($3 >= 1000 && $3 != 65534) print $1}' /etc/passwd
// https://stackoverflow.com/questions/33294932/parsing-variable-from-delimited-file
// Because of my uncle's hackery we will not by filtering out below 65543
fn _get_users(){

}

fn _filter_passwd(){
    let file_in = File::open("/etc/passwd").expect("No such file");

    let reader = BufReader::new(file_in);

    for line in reader.lines(){
        for value in line.unwrap().split(':') {
            println!("{}", value);
        }
    }
}