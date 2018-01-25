extern crate bmp;
mod lib;

use std::env;
use lib::{parse_input, read_bmp_list};

fn main() {
    // parse user input
    let mut src_dir = String::new();
    if parse_input(&mut src_dir, env::args()) != true {
        println!("Invalid arguments!!");
        return;
    }

    // get image file path list
    let mut path_list = Vec::new();
    if read_bmp_list(&mut path_list, src_dir) != true {
        println!("Cannot read bmp files!!");
        return;
    }

    // get image info. from file
    for path in path_list {
        // get image info
        let (width, height) = get_image_info(&path);
        // get file name
        let mut name = String::new();
        if get_image_name(&mut name, path) != true {
            continue;
        }

        let str_code = format!("U16\tcbits_{0}[{1}*{2}];", &name, width, height);
        println!("{}", str_code);
    }
}

fn get_image_info(path: &str) -> (u32, u32) {
    let image = match bmp::open(path) {
        Ok(x) => x,
        Err(e) => {
            println!("Cannot open {0} because {1}", path, e);
            return (0, 0);
        },
    };

    (image.get_width(), image.get_height())
}

fn get_image_name(name: &mut String, path: String) -> bool {
    // remove directory name
    let mut path_vec: Vec<&str> = path.split('/').collect();
    let file_name;
    if let Some(x) = path_vec.pop() {
        file_name = x;
    }
    else {
        return false;
    }

    // remove extensiton
    let file_name_string = file_name.to_string();
    let file_name_vec: Vec<&str> = file_name_string.split('.').collect();

    // fill result
    name.push_str(file_name_vec[0]);

    return true
}
