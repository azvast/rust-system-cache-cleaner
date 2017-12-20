extern crate clap;

use clap::{Arg, App};
mod cleaner;
mod conf;
mod utils;

fn main() {
	// Defines command line arguments.
	let matches = App::new("Cache Cleaner")
		.version("0.1.0")												// version
		.author("Dakota James Owen Keeler <Bearzrobotics@gmail.com>")	// name
		.about("This is a simple util to clean cache up on my system")
		.arg(Arg::with_name("debug")
			.short("d")
			.long("debug")
			.takes_value(false)
			.help("This sets the debuing flag to true, printing out all debuging info. To set the flag -d t or --debug t"))
		.get_matches(); 
		
	let debug_flag = matches.value_of("debug");
	if debug_flag == None{
		println!("Debug value: False");
		cleaner::create_log_file(false);
		cleaner::delete_user_cache(false);

		if utils::am_root() == true{
			cleaner::delete_system_cache(false);
		}
		
	}else{
		println!("Debug value: True");
		cleaner::create_log_file(true);
		cleaner::delete_user_cache(true);

		if utils::am_root() == true{
			cleaner::delete_system_cache(true);
		}else {
			println!("Not running as root");
		}
	}
		
}


