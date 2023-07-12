use std::fs;

pub fn copy(from: &String, to: &String) {
    match fs::copy(from, to) {
        Ok(_) => println!("{from} copy to {to}"),
        Err(error) => eprintln!("{}", error),
    }
}