use std::fs;

pub fn rename(before: &String, after: &String) {
    match fs::rename(before, after) {
        Ok(_) => println!("{before} rename to {after}"),
        Err(error) => eprintln!("{}", error),
    }
}