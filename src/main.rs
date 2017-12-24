
extern crate clap;

use clap::{Arg, App, AppSettings};
mod cleaner;
mod conf;
mod utils;

fn main() {
	// Defines command line arguments.
	let matches = App::new("Cache Cleaner")
		.version("0.1.0")												// version
		.author("Dakota James Owen Keeler <Bearzrobotics@gmail.com>")	// name
		.about("This is a simple util to clean cache up on my system")
		.setting(AppSettings::ColorAuto)
		.arg(Arg::with_name("debug")
			.short("d")
			.long("debug")
			.takes_value(false)
			.help("This sets the debuing flag to true, printing out all debuging info."))
		.arg(Arg::with_name("Delete user cache")
			.short("s")
			.long("delete-user")
			.takes_value(false)
			.help("This Deletes just the user cache"))
		.arg(Arg::with_name("Delete system cache")
			.short("D")
			.long("delete-system")
			.takes_value(false)
			.help("This Deletes the system cache <Must be root to run>"))
		.arg(Arg::with_name("all")
			.short("a")
			.long("all")
			.takes_value(false)
			.help("This deletes both system and user cache <Must be root to run>"))
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Sets a custom config file")
			.takes_value(true))
		.get_matches();		

	//let all_flag = matches.value_of("all");
	//let delete_system_cache_flag = matches.value_of("Delete system cache")
	//let delete_user_cache_flag = matches.value_of("Delete user cache")
	//let config_flag = matches.value_of("config").unwrap();

	if matches.is_present("debug"){
		println!("Debug value: True");
		utils::create_log_file(true);
		cleaner::delete_user_cache(true);

		if utils::am_root() == true{
			cleaner::delete_system_cache(true);
		}else {
			println!("Not running as root");
		}
	}else{
		utils::create_log_file(false);
		cleaner::delete_user_cache(false);

		if utils::am_root() == true{
			cleaner::delete_system_cache(false);
		}
	}
		
}


