#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

extern crate regex;

use std::fs;
use std::path::Path;

mod range;
mod streak;

// 1. list files in directory
//

fn list_directory(directory: &Path) {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_files() {
        let resource_dir = Path::new("/Users/eddiehoyle/Code/rust/streaker/resources");
        let paths = fs::read_dir(resource_dir).unwrap();


}

}
