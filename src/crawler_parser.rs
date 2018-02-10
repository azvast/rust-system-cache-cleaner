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
use std::fs::File;
use std::io::BufReader;
use std::process;
use utils;


pub fn get_crawler_files(mode: u8, crawler_dir: String) -> Vec<String>{
    let mut crawler_files = Vec::new();
    let paths = fs::read_dir(&crawler_dir).unwrap();

    for path in paths {
        let pth = path.unwrap().path().file_name().unwrap().to_string_lossy().into_owned();
        let mut crawler_path = crawler_dir.clone();
        crawler_path.push_str(&pth);

        if crawler_path != " ".to_string(){
            crawler_files.push(crawler_path);
        }
    }
            
    if mode == 1 {
        for i in 0..crawler_files.len(){
            println!("{:?}", crawler_files[i]);
        }
    }
    crawler_files
}

pub fn crawler_interater(crawler_files: Vec<String>){
    for i in 0..crawler_files.len(){
        println!("crawlers: {}", crawler_files[i]);
        //crawler_parser(cralwer_files[i].to_string());
        println!(" ");
    }

}