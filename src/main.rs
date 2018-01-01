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
#[macro_use] extern crate clap;
#[macro_use] extern crate log;
extern crate simplelog;

use clap::{Arg, App, AppSettings};
use std::fs::File;
use simplelog::*;
// custom includes
mod cleaner;
mod conf;
mod utils;

fn main() {

	// inits logger

    //error!("Bright red error");
    //info!("This only appears in the log file");
    //debug!("This level is currently not enabled for any logger");
	
	let log_path = utils::get_log_path(0);
	CombinedLogger::init(
		vec![
			TermLogger::new(LogLevelFilter::Warn, Config::default()).unwrap(),
			WriteLogger::new(LogLevelFilter::Info, Config::default(), File::create(log_path).unwrap())
		]
	).unwrap();


	// Defines command line arguments.
	let matches = App::new("Cache Cleaner")
		.version(crate_version!())												// version
		.author(crate_authors!())	// name
		.about("This is a simple util to clean cache up on my system")
		.setting(AppSettings::ColorAuto)
		.arg(Arg::with_name("delete_all_cache")
            .short("a")
            .long("delete_all")
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
			.takes_value(true)
			.help("Sets level of Verbose <1 = debug, 2 = verbose>"))
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
		let verbose_mode = value_t!(matches.value_of("verbose"), u8).unwrap_or_else(|e| e.exit());
		info!("Verbose value: True");
		all(verbose_mode)
	}else if  matches.is_present("delete_all_cache"){
		all(0);
	}else{
		// By Defualt deletes user cache.
		cleaner::delete_user_cache(0);
	}		
}

// This is made to make the if statments above easier to read
fn all(mode: u8){
	cleaner::delete_user_cache(mode);
	cleaner::delete_system_cache(mode);
}