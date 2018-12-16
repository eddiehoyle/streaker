//#![allow(dead_code)]
//#![allow(unused_variables)]
//
//extern crate streaker;
//extern crate clap;
//
//mod range;
//mod streak;
//mod pattern;
//
//use clap::{App, Arg};
//use std::env;
//use std::path;
//use std::fs;
//

fn main() {}

//fn main() {
//
//    let matches = App::new("Streaker")
//        .version("1.0")
//        .author("Eddie Hoyle")
//        .about("Search for file sequences")
//        .arg(Arg::with_name("directory")
//            .short("d")
//            .index(1)
//            .required(true))
//        .get_matches();
//
//
//    if let Some(dir) = matches.value_of("directory") {
//        let path = env::current_dir()
//            .unwrap()
//            .join(dir);
//        if let Ok(full_directory_path) = fs::canonicalize(path) {
//            if full_directory_path.is_dir() {
//                seek_directory(full_directory_path.as_path());
//            }
//        }
//
////        let abs_path = fs::canonicalize(path);
////        match abs_path {
////            Ok(p) => {
////                let s = seek(&p.as_path());
////                println!("Streak: {}", s);
////            },
////            Err(_) => {
////                eprintln!("Path does not exist: {}", dir);
////                process::exit(1);
////            }
////        };
//    }
//}
//
//fn seek_directory(path: &path::Path) -> Vec<streak::Streak> {
//    println!("Querying: {}", path.to_str().unwrap());
//    if let Ok(files) = fs::read_dir(path) {
//        let files: Vec<_> = files.collect();
//    }
//    Vec::new()
//}
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
