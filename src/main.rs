extern crate bmp;

mod util;

use std::env;
use util::*;
use std::process::Command;
use std::fs::File;
use std::io::Write;

fn main() {
    // parse user input
    let mut src_dir = String::new();
    let mut des_dir = String::new();
    if parse_input(&mut src_dir, &mut des_dir, env::args()) != true {
        println!("Invalid arguments!!");
        return;
    }

    // get image file path list
    let mut img_path_list = Vec::new();
    if read_bmp_list(&mut img_path_list, &src_dir) != true {
        println!("Cannot read bmp files!!");
        return;
    }

    // prepare output directory
    copy_sub_dirs(&src_dir, &des_dir);

    // fill output contents
    let mut cbit_str = String::new();
    for img_path in img_path_list {
        // 1. make raw file
        // - target path
        //  : remove root path dir of input
        let root_dir_offset = img_path.find('/').unwrap();
        let (_, last_path) = img_path.split_at(root_dir_offset);
        //  : remove extension and add .h
        let extension_offset = last_path.find('.').unwrap();
        let (target_file, _) = last_path.split_at(extension_offset);
        let target_file_path = format!("{0}{1}.h", &des_dir, &target_file);
        // - make raw file
        Command::new("bmpToRawC")
                .arg(&img_path).arg(&target_file_path)
                .arg("1").arg("1").arg("0").arg("2")
                .spawn()
                .expect("failed to execute process");;

        // 2. info header
        // get image info
        let (width, height) = get_image_info(&img_path);
        // get file name
        let mut name = String::new();
        if get_image_name(&mut name, &img_path) != true {
            continue;
        }

        cbit_str.push_str(&format!("U16\tcbits_{0}[{1}*{2}];", &name, width, height));
    }

    let mut file_buffer = match File::create("out_image.h") {
        Ok(x) => x,
        Err(_) => {
            println!("Cannot write info header file");
            return;
        },
    };
    file_buffer.write(cbit_str.as_bytes());
}
