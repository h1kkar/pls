use std::fs::{
    remove_dir,
    remove_file,
};

pub fn delete(path: &[String]) {
    for target in path {
        if &target[target.len()-1..] == "/" {
            match remove_dir(target) {
                Ok(_) => println!("dir {} is delete", &target[..target.len()-1]),
                Err(error) => eprintln!("{}", error),
            }
        } else {
            match remove_file(target) {
                Ok(_) => println!("file {} is delete", &target),
                Err(error) => eprintln!("{}", error),
            }
        }
    }
}