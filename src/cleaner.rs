use std::io::prelude::*;
use std::fs;
use std::fs::File;

/// This function checks if a file exist 
pub fn check_if_path_exist(path: &str) -> bool{
	fs::metadata(path).is_ok()
}


/// This first checks if the log file exist
/// if not, it will create it. For now it wil create
/// the file in /home/dakota/clear_cache_logs .
// Ideally it should be under /varhttps://github.com/rust-lang/rust/issues?utf8=%E2%9C%93&q=is%3Aissue+is%3Aopen+static
pub fn create_log_file(){
	let log_path = "/home/dakota/clear_cache_logs";

	if check_if_path_exist(log_path) == true{
		println!("Log file already exist");
	}else {
		let mut log_file = File::create(log_path).expect("Unable to create file");
		log_file.write_all(b"File Created").unwrap();
		log_file.sync_all().unwrap();
	}	
}

/// This function delete users cache
/// Firefox
/// Cache dir
pub fn delete_user_cache(){
	// this is used for deleting whole direcotries
	let mut path_dir_vec = Vec::new();
	path_dir_vec.push("/home/dakota/.mozilla/firefox/Crash Reports");
	// thumbnails
	path_dir_vec.push("/home/dakota/.cache/thumbnails/fail/gnome-thumbnail-factory");
	path_dir_vec.push("/home/dakota/.thumbnails/normal");
	path_dir_vec.push("/home/dakota/.cache/thumbnails/large");
	// transmission
	path_dir_vec.push("/home/dakota/.cache/transmission/favicons");
	// FRC-Utilities
	path_dir_vec.push("/home/dakota/.cache/FRC Utilities");
	// Trash
	path_dir_vec.push("/home/dakota/.local/share/Trash/files");
	path_dir_vec.push("/home/dakota/.local/share/Trash/info");
	path_dir_vec.push("/home/dakota/.local/share/Trash/expunged");
	// Misc stuff
	path_dir_vec.push("/home/dakota/.cache/fontconfig");
	path_dir_vec.push("/home/dakota/.cache/keyring-YQPR3Y");
	path_dir_vec.push("/home/dakota/.cache/gnome-screenshot");
	path_dir_vec.push("/home/dakota/.cache/QtProject");
	path_dir_vec.push("/home/dakota/.cache/gstreamer-1.0");
	// Google Chrome
	path_dir_vec.push("/home/dakota/.cache/google-chrome/Default/Cache");

	for x in &path_dir_vec{
		if check_if_path_exist(x) == true{
			fs::remove_dir_all(x).expect("Failded to delete");
			println!("Deleted Dir: {}", x);
		} else {
			println!("Dir didn't exist: {}", x);
		}
	}

	// this used for deleting invidual files
	let mut path_file_vec = Vec::new();
	// clean up firefox data");
	path_file_vec.push("/home/dakota/.mozilla/firefox/fngozv6t.default/content-prefs.sqlite");
	path_file_vec.push("/home/dakota/.mozilla/firefox/fngozv6t.default/formhistory.sqlite");
	path_file_vec.push("/home/dakota/.mozilla/firefox/fngozv6t.default/storage-sync.sqlite");
	path_file_vec.push("/home/dakota/.mozilla/firefox/fngozv6t.default/places.sqlite");
	path_file_vec.push("/home/dakota/.mozilla/firefox/fngozv6t.default/storage.sqlite");
	path_file_vec.push("/home/dakota/.mozilla/firefox/fngozv6t.default/webappsstore.sqlite");
	path_file_vec.push("/home/dakota/.mozilla/firefox/fngozv6t.default/permissions.sqlite");
	path_file_vec.push("/home/dakota/.mozilla/firefox/fngozv6t.default/kinto.sqlite");
	path_file_vec.push("/home/dakota/.mozilla/firefox/fngozv6t.default/favicons.sqlite");
	path_file_vec.push("/home/dakota/.mozilla/firefox/fngozv6t.default/cookies.sqlite");
	//Google Chrome
	path_file_vec.push("/home/dakota/.config/google-chrome/Local State");
	// Vlc
	path_file_vec.push("/home/dakota/.config/vlc/vlc-qt-interface.conf");
	path_file_vec.push("/home/dakota/.config/vlc/vlc-qt-interface.conf");
	path_file_vec.push("/home/dakota/.config/vlc/vlc-qt-interface.conf");
	// Transmission 
	path_file_vec.push("/home/dakota/.config/transmission/stats.json");
	//Flash
	path_file_vec.push("/home/dakota/.adobe/Flash_Player/AssetCache/XAZN9C5F");
	path_file_vec.push("/home/dakota/.macromedia/Flash_Player/#SharedObjects/LTN8XVF7");
	path_file_vec.push("/home/dakota/.macromedia/Flash_Player/macromedia.com/support/flashplayer/sys/settings.sol");
	// Tracker
	path_file_vec.push("/home/dakota/.cache/tracker/ontologies.gvdb");
	path_file_vec.push("/home/dakota/.cache/tracker/first-index.txt");
	path_file_vec.push("/home/dakota/.cache/tracker/db-version.txt");
	path_file_vec.push("/home/dakota/.cache/tracker/meta.db");
	path_file_vec.push("/home/dakota/.cache/tracker/meta.db-shm");
	path_file_vec.push("/home/dakota/.cache/tracker/locale-for-miner-apps.txt");
	path_file_vec.push("/home/dakota/.cache/tracker/meta.db-wal");
	path_file_vec.push("/home/dakota/.cache/tracker/last-crawl.txt");
	path_file_vec.push("/home/dakota/.cache/tracker/db-locale.txt");
	path_file_vec.push("/home/dakota/.cache/tracker/parser-sha1.txt");

	for x in &path_file_vec{
		if check_if_path_exist(x) == true{
			fs::remove_file(x).expect("Failded to delete");
			println!("Deleted File: {}", x);
		} else {
			println!("File didn't exist: {}", x);
		}
	}


}