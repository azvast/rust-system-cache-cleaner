/*
KNOW ALL MEN BY THESE PRESENTS: 'i': man [Dakota James Owen Keeler]
copyright this software in the year of our lord 2017 under the Apache 2
Contact: bearzrobotics@gmail.com

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the Apache 2 License
along with this program. .

live honorably, harm no one, give to each his own.
*/
use std::fs;
use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};
use std::process;
use utils;

fn indent(size: usize) -> String {
    const INDENT: &'static str = "    ";
    (0..size).map(|_| INDENT)
            .fold(String::with_capacity(size*INDENT.len()), |r, s| r + s)
}

pub fn xml_parser(xml_files: String) {
    let file = {
        if utils::check_if_path_exist(&xml_files) == true{
			File::open(xml_files).expect("make sure you passed a valid direcoty")
		}else{
			println!("Make sure you passed a valid direcoty. You must add the / or \\ Path didn't exits: {}", xml_files);
			process::exit(0)
		}
    };
     
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    let mut depth = 0;
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement {name, ..}) => {
                println!("{}+{}", indent(depth), name);
                depth += 1;
            }
            Ok(XmlEvent::EndElement {name}) => {
                depth -= 1;
                println!("{}-{}", indent(depth), name);
            }
            Ok(XmlEvent::Characters(parser)) => { // this displays the info in code like <name></name> but not <run d="" />
                println!("{}{}", indent(depth), parser);
            }

            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
}

/// This funtion goes out and make a list of paths of xml files.
/// The reason for this, is so you can just drop in you own or delete 
/// the undesired xmls with out having to recompile.
pub fn get_xml_files(mode: u8, xml_dir: String) -> Vec<String> {
	let mut xml_files = Vec::new();
	let paths = fs::read_dir(&xml_dir).unwrap();

    for path in paths {
        let pth = path.unwrap().path().file_name().unwrap().to_string_lossy().into_owned();
        let mut xml_path = xml_dir.clone();
        xml_path.push_str(&pth);
        // lets not capture the junk accouts
        if xml_path != " ".to_string(){
            xml_files.push(xml_path);
        }
    }
    if mode == 1 {
        for i in 0..xml_files.len(){
            println!("{:?}", xml_files[i])
        }
    }
    xml_files
}

pub fn xml_interater(xml_files: Vec<String>){
    for i in 0..xml_files.len(){
        //println!("Xml: {}", xml_files[i]);
        xml_parser(xml_files[i].to_string());
        println!(" ");
    }
}