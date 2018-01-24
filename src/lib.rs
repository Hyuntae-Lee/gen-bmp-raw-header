use std::env;
use std::fs;

pub fn parse_input(out_src: &mut String, out_des: &mut String, mut args: env::Args) -> bool {
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

    // third argument is a directory of source images
    if let Some(x) = args.next() {
        out_des.push_str(&x);
    }
    else {
        println!("Invalid arguments!!");
        return false;
    }

    true
}

pub fn read_bmp_list(out_list: &mut Vec<String>, dir_path: String) -> bool {
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
            if read_bmp_list(&mut path_list, path_str.to_string()) == true {
                continue;
            }
        }

        // homogenize expression
        path_list.push(path_str.to_string().replace("\\", "/"));
    }

    out_list.append(&mut path_list);

    true
}
