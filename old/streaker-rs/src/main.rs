#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

extern crate streaker;
extern crate clap;
extern crate regex;
extern crate indexmap;

use std::env;
use std::path::{Path};
use std::fs;
use regex::Regex;
use clap::{App, Arg};
use streaker::streak::{Streak};
use indexmap::IndexSet;

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

    let mut streaks : Vec<Streak> = Vec::new();

    if let Ok(files) = fs::read_dir(path) {
        let re = Regex::new(r"^(?P<name>\w+).(?P<frame>\d+).(?P<ext>\w+)$").unwrap();
        for file in files {
            if let Some(cap) = re.captures(file.unwrap().file_name().to_str().unwrap()) {

                let name = cap.name("name").unwrap().as_str();
                let frame = cap.name("frame").unwrap().as_str();
                let ext = cap.name("ext").unwrap().as_str();
                let padding = frame.len();

                let mut found = false;
                for streak in &mut streaks {
                    if streak.is_match(&String::from(name),
                                       &String::from(ext),
                                       padding as u32) {
                        streak.frames_mut().insert(frame.parse::<u32>().unwrap());
                        found = true;
                    }
                }

                if !found {
                    let mut frames = IndexSet::new();
                    frames.insert(frame.parse::<u32>().unwrap());
                    streaks.push(Streak::new(String::from(name),
                                             String::from(ext),
                                             padding as u32,
                                             frames));
                }
            }
        }
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_seek_directory() {

    }

}