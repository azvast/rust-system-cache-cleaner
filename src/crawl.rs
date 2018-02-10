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

// This craws based on a folder filled with crawler files. To create new places
// for it to craw, just make a new file.

use crawler_parser;

// The only public functin should be the run crawler function.
pub struct crawler {
    pub crawler_path:       String,      //dir containing crawler files
    cached_files:       Vec<String>, //Contains string of paths for files
    cached_dirs:        Vec<String>, //Contains string of paths for folders
}

/// The control byte has a cupple modes
///  0 - crawl user files (defualt)
///  1 - crawl system files only
///  2 - crawl system and user files
///  3 - 0 and delete user files
///  4 - 1 and delete system files
///  5 - 2 and delete files
impl crawler {
    pub fn new(crawler_path: String) -> crawler{
        crawler{crawler_path: crawler_path, cached_files: vec![], cached_dirs: vec![]}
    }

    pub fn craw(&mut self, control_byte: u8, mode: u8){ 
        // setup work
        let crawler_path = self.crawler_path.clone();
        let crawler_files = crawler_parser::get_crawler_files(mode, crawler_path);
        crawler_parser::crawler_interater(crawler_files);
        
        if control_byte == 0 {
            self.parse_user_files(mode);
        }else if control_byte == 1{
            self.parse_system_files(mode);
        }else if control_byte == 2{
            self.parse_user_files(mode);
            self.parse_system_files(mode);
        }else if control_byte == 3{
            self.parse_user_files(mode);
        }else if control_byte == 4{
            self.parse_system_files(mode);
        }else if control_byte == 5{
            self.parse_user_files(mode);
            self.parse_system_files(mode);
        }else{
            println!("crawler was not passed a valid option");
        }
    }

    fn parse_user_files(&mut self, mode: u8){
        if mode == 1 {
            println!("Parse User files");
        }
    }

    fn parse_system_files(&mut self, mode: u8){
        if mode == 1 {
            println!("Parse system files");
        }
    }
}