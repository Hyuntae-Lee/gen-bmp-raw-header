mod lib;

use std::env;
use lib::{parse_input, read_bmp_list};

fn main() {
    // parse user input
    let mut src_dir = String::new();
    let mut des_dir = String::new();
    if parse_input(&mut src_dir, &mut des_dir, env::args()) != true {
        println!("Invalid arguments!!");
        return;
    }

    // get image file path list
    let mut path_list = Vec::new();
    if read_bmp_list(&mut path_list, src_dir) != true {
        println!("Cannot read bmp files!!");
        return;
    }



    // test
    for path in path_list {
        println!("{:?}", path);
    }
}
