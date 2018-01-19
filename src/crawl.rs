use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use standard_paths;

// The only public functin should be the run crawler function.
pub struct Crawler {
    pub starting_path:  String,
    pub base:           String,
    cached_files:       Vec<String>, //Contains string of paths
}

/// The control byte has a cupple modes
///  0 - crawl user files (defualt)
///  1 - crawl system files only
///  2 - crawl system and user files
///  3 - 0 and delete user files
///  4 - 1 and delete system files
///  5 - 2 and delete files
impl Crawler {
    pub fn new(starting_path: String, base: String) -> Crawler{
        Crawler{starting_path: starting_path, base: base, cached_files: vec![]}
    }

    pub fn craw(&mut self, control_byte: u8, mode: u8){ 
        if control_byte == 0 {
            self.parse_user_files();
        }else if control_byte == 1{
            self.parse_system_files();
        }else if control_byte == 2{
            self.parse_user_files();
            self.parse_system_files();
        }else if control_byte == 3{
            self.parse_user_files();
        }else if control_byte == 4{
            self.parse_system_files();
        }else if control_byte == 5{
            self.parse_user_files();
            self.parse_system_files();
        }else{
            println!("Crawler was not passed a valid option");
        }
    }

    #[cfg(target_os = "linux")]
    fn parse_system_files(&mut self){

    }

    #[cfg(target_os = "windows")]
    fn parse_system_files(&mut self){
        
    }

    #[cfg(target_os = "linux")]
    fn parse_user_files(&mut self){

    }

    #[cfg(target_os = "windows")]
    fn parse_user_files(&mut self){
        
    }
}