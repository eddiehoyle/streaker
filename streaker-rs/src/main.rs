#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

extern crate streaker;
extern crate clap;
extern crate regex;

//mod range;
//mod streak;

use clap::{App, Arg};
use std::env;
use regex::Regex;
use std::path::{Path};
use std::fs;


fn main() {

    let matches = App::new("Streaker")
        .version("1.0")
        .author("Eddie Hoyle")
        .about("Search for file sequences")
        .arg(Arg::with_name("directory")
            .short("d")
            .index(1)
            .required(true))
        .get_matches();


    if let Some(dir) = matches.value_of("directory") {
        let path = env::current_dir()
            .unwrap()
            .join(dir);
        if let Ok(full_directory_path) = fs::canonicalize(path) {
            if full_directory_path.is_dir() {
                print!("Scanning directory: {}", full_directory_path.to_str().unwrap());
                seek_directory(full_directory_path.as_path());
            }
        };
    }
}

fn seek_directory(path: &Path) {
    println!("Querying: {}", path.to_str().unwrap());
    if let Ok(files) = fs::read_dir(path) {
        for file in files {
            let file = file.unwrap();
            let re = Regex::new(r"^(?P<name>\d+).(?P<frame>\d+).(?P<ext>\d+)$").unwrap();
            if let Some(cap) = re.captures(file.file_name().to_str().unwrap()) {
                let name = cap.name("name").unwrap();
                let frame = cap.name("frame").unwrap();
                let ext = cap.name("ext").unwrap();
                print!("{} {} {}", name, frame, ext);
            }
//            let re = Regex::new(r"(\d+)").unwrap();
//            print!("{}", file.file_name().to_str().unwrap());
//            for mat in re.find_iter(file.file_name().to_str().unwrap()) {
//                print!(", {}", mat.as_str());
//            }
//            print!("\n");
        }
    }
}

////
////fn seek(path: &path::Path) -> Streak {
////
////    let mut padding = 1;
////    let mut frames : Vec<u32> = Vec::new();
////    let pattern = String::new();
////    if let Ok(files) = fs::read_dir(path) {
////        let files: Vec<_> = files.collect();
////        for entry in files {
////            if let Ok(file) = entry {
////                let filename = &file.file_name();
////                let split = filename.to_str().unwrap();
////                let tokens : Vec<_> = split.split(".").collect();
////                let number = tokens.get(1).unwrap();
////                if let Ok(pad) = get_padding(number) {
////                    frames.push(number.parse::<u32>().unwrap());
////                    if pad > padding {
////                        padding = pad;
////                    }
////                }
////                let pattern = Pattern::new(String::from(*tokens.get(0).unwrap()),
////                                           String::from(*tokens.get(2).unwrap()));
////                return Streak::new(pattern, range::Range::new(frames), padding);
////            }
////        }
////    }
////    Streak::new(Pattern::new(String::from("not"),String::from("found")),
////                range::Range::new(frames), padding)
////}
