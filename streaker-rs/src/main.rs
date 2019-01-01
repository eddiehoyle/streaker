#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

extern crate streaker;
extern crate clap;
extern crate regex;

use clap::{App, Arg};
use std::env;
use regex::Regex;
use std::path::{Path};
use std::fs;
use std::collections::{HashMap, BTreeSet};
use streaker::streak::{Streak};

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
                println!("Scanning directory: {}", full_directory_path.to_str().unwrap());
                seek_directory(full_directory_path.as_path());
            }
        };
    }
}

fn seek_directory(path: &Path) {
//    let mut map : HashMap<String, BTreeSet<u32>> = HashMap::new();
    let mut streaks : BTreeSet<Streak> = BTreeSet::new();
    if let Ok(files) = fs::read_dir(path) {
        for file in files {
            let file = file.unwrap();
            let re = Regex::new(r"^(?P<name>\w+).(?P<frame>\d+).(?P<ext>\w+)$").unwrap();
            if let Some(cap) = re.captures(file.file_name().to_str().unwrap()) {

                let name = cap.name("name").unwrap().as_str();
                let frame = cap.name("frame").unwrap().as_str();
                let ext = cap.name("ext").unwrap().as_str();
                let padding = frame.len();

                for streak in &streaks {

                }
//                let mut found = false;
//                for mut streak in &streaks {
//                    if streak.name() == name &&
//                        streak.ext() == ext &&
//                        streak.padding() <= padding as u32 {
//                        let mut frames = streak.frames_mut();
////                        frames.insert(frame.parse::<u32>().unwrap());
//                        found = true;
//                    }
//                }

                let found = false;
                if !found {
                    let mut frames = BTreeSet::new();
                    frames.insert(frame.parse::<u32>().unwrap());
                    streaks.insert(Streak::new(String::from(name),
                                               String::from(ext),
                                               padding as u32,
                                               frames));
                }

//                // Only match on name
//                if !map.contains_key(&name.to_string()) {
//                    map.insert( name.to_string(), BTreeSet::new());
//                }
//                map.get_mut(&name.to_string())
//                    .unwrap()
//                    .insert( frame.parse::<u32>().unwrap());
            }
        }
    }

    println!("Streaks: {}", streaks.len());

//    for iter in map.iter() {
//        let (key, value) = &iter;
//    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_seek_directory() {

    }

}