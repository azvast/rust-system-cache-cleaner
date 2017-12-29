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
		.arg(Arg::with_name("delete_all_cache")
            .short("a")
            .long("delete_allr")
            .help("Deletes both user and system cache <Must be root>")
			.takes_value(false))
		.arg(Arg::with_name("delete_system_cache")
            .short("s")
            .long("delete_system")
            .help("Delete System cache, Must be root")
			.takes_value(false))
		.arg(Arg::with_name("delete_user_cache")
            .short("u")
            .long("delete_user")
            .help("Delete user cache, If not running as root it will only delete the current users. \r\n <If no options supplied this is the defualt>")
			.takes_value(false))
		.arg(Arg::with_name("verbose")
			.short("v")
			.long("verbose")
			.takes_value(false)
			.help("This sets the verbose flag to true, printing out verbose info. Less than debug though.\r\n Don't run with debug"))
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
	}else if  matches.is_present("delete_all_cache"){
		utils::create_log_file(0);
		cleaner::delete_user_cache(0);

		if utils::am_root() == true{
			println!("Running as root");
			cleaner::delete_system_cache(0);
		}else {
			println!("Not running as root");
		}
	}else{
		// By Defualt deletes user cache.
		utils::create_log_file(0);
		cleaner::delete_user_cache(0);
	}		
}


