use std::env;
use std::fs;
use std::path::{PathBuf};
use bmp;

pub fn parse_input(out_src: &mut String, out_dest: &mut String, mut args : env::Args) -> bool {
    // first argument is excuation file
    args.next();

    // second argument is a directory of source images
    if let Some(x) = args.next() {
        out_src.push_str(&x);
    }
    else {
        println!("Invalid arguments!!");
        return false;
    }

    // third argument is a output directory
    if let Some(x) = args.next() {
        out_dest.push_str(&x);
    }
    else {
        println!("Invalid arguments!!");
        return false;
    }

    true
}

pub fn copy_sub_dirs(src_dir_str: &str, des_dir_str: &str) -> bool {
    // read file list
    let dir_entry_result_itr;
    if let Ok(x) = fs::read_dir(src_dir_str) {
        dir_entry_result_itr = x;
    }
    else {
        return false;
    }

    // check file list
    for dir_entry_result in dir_entry_result_itr {
        // get path
        let path_buf;
        if let Ok(x) = dir_entry_result {
            path_buf = x.path();
        }
        else {
            continue;
        }

        // only handle directory
        if path_buf.is_dir() == false {
            continue;
        }

        // compose sub path
        let mut des_sub_dir = PathBuf::from(des_dir_str);
        let mut src_sub_dir = path_buf.as_path();

        // get relative path
        let sub_rel_path;
        if let Ok(x) = src_sub_dir.strip_prefix(src_dir_str) {
            sub_rel_path = x;
        }
        else {
            continue;
        }

        // append sub-dir to destination root path
        des_sub_dir.push(sub_rel_path);

        // create sub path
        match fs::create_dir(&des_sub_dir) {
            _ => {
                // do nothing
            }
        }

        // also scan sub directories
        if let Some(x) = path_buf.to_str() {
            copy_sub_dirs(x, &des_sub_dir.to_str().unwrap());
        }
    }

    true
}

pub fn read_bmp_list(out_list : &mut Vec<String>, dir_path : &str) -> bool {
    let mut path_list = Vec::new();

    // read file list
    let path_ret_list;
    if let Ok(x) = fs::read_dir(dir_path) {
        path_ret_list = x;
    }
    else {
        return false;
    }

    // filter file list
    for path_ret in path_ret_list {
        let path_buf;
        if let Ok(x) = path_ret {
            path_buf = x.path();
        }
        else {
            continue;
        }

        // string format
        let path_str;
        if let Some(x) = path_buf.to_str() {
            path_str = x;
        }
        else {
            continue;
        }

        // only bmp file is acceptable
        if let Some(extension) = path_buf.extension() {
            if extension != "bmp" {
                continue;
            }
        }
        else {
            // scan sub directries also
            if read_bmp_list(&mut path_list, path_str) == true {
                continue;
            }
        }

        // homogenize expression
        path_list.push(path_str.to_string().replace("\\", "/"));
    }

    out_list.append(&mut path_list);

    true
}

pub fn get_image_info(path: &str) -> (u32, u32) {
    let image = match bmp::open(path) {
        Ok(x) => x,
        Err(e) => {
            println!("Cannot open {0} because {1}", path, e);
            return (0, 0);
        },
    };

    (image.get_width(), image.get_height())
}

pub fn get_image_name(name: &mut String, path_str: &str) -> bool {
    let path = String::from(path_str);    

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
