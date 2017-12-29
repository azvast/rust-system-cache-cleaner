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
			.help("This sets the debuing flag to true, printing out all debuging info.\n Don't run with verbose. This will print out all info that verbose does."))
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Sets a custom config file")
			.takes_value(true))
		.arg(Arg::with_name("delete_system_cache")
            .short("D")
            .long("delete_system")
            .help("Delete System cache, Must be root")
			.takes_value(true))
		.arg(Arg::with_name("verbose")
			.short("v")
			.long("verbose")
			.takes_value(false)
			.help("This sets the verbose flag to true, printing out verbose info. Less than debug though.\n Don't run with debug"))
		.get_matches();		

	//let all_flag = matches.value_of("all");
	//let delete_system_cache_flag = matches.value_of("Delete system cache")
	//let delete_user_cache_flag = matches.value_of("Delete user cache")
	//let config_flag = matches.value_of("config").unwrap();

	// the first value is a u8 for mode
	// 0 = no mode
	// 1 = debug
	// 2 = verbose

	if matches.is_present("verbose"){
		println!("Verbose value: True");
		utils::create_log_file(2);
		cleaner::delete_user_cache(2);

		if utils::am_root() == true{
			println!("Running as root");
			cleaner::delete_system_cache(2);
		}else {
			println!("Not running as root");
		}
	}else if matches.is_present("debug"){
		println!("Debug value: True");
		utils::create_log_file(1);
		cleaner::delete_user_cache(1);

		if utils::am_root() == true{
			println!("Running as root");
			cleaner::delete_system_cache(1);
		}else {
			println!("Not running as root");
		}
	}else{
		utils::create_log_file(0);
		cleaner::delete_user_cache(0);

		if utils::am_root() == true{
			println!("Running as root");
			cleaner::delete_system_cache(0);
		}else {
			println!("Not running as root");
		}
	}		
}


