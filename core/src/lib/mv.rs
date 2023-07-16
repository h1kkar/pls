use std::fs;

pub fn rename(before: &String, after: &String) {
    match fs::rename(before, after) {
        Ok(_) => {
            if check(after) {
                println!("{before} move to {after}")
            } else {
                println!("{before} rename to {after}")
            }
        },
        Err(error) => eprintln!("{}", error),
    }
}

fn check(file: &String) -> bool {
    let bytes = file.as_bytes();
    for (_i, &item) in bytes.iter().enumerate() {
        if item == b'/' {
            return true;
        }
    }
    
    false
}