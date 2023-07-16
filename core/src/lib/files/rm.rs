use std::{
    io::{
        self,
        prelude::*
    },
    str::from_utf8,
    process::Command,
    fs::{
    remove_dir,
    remove_file,
    }
};

pub fn delete(path: &[String]) {
    for target in path {
        match remove_dir(target) {
            Ok(_) => println!("dir {} deleted", &target),
            Err(_) => {
                match remove_file(target) {
                    Ok(_) => println!("file {} deleted", &target),
                    Err(_) => {
                        println!("you really want to remove the non empty {} dir?", &target);
                        print!("yes or Enter for removal: ");
                        io::stdout().flush().ok().expect("err");

                        let mut choise = String::new();
                        let _ = io::stdin().read_line(&mut choise);
                        let choise = String::from(choise);

                        match &choise[..] {
                            "yes\n" | "yep\n" | "y\n" | "\n" => {

                                let cmd = String::from("ls ") + &target;
                                let files = Command::new("sh")
                                    .arg("-c")
                                    .arg(cmd)
                                    .output()
                                    .expect("err")
                                    .stdout;

                                let files = match from_utf8(&files) {
                                    Ok(out) => out,
                                    Err(error) => panic!("{}", error),

                                };
                                let mut files: Vec<&str> = files.split('\n').collect();
                                files.remove(files.len()-1);

                                for file in &files {
                                    let f = String::from("") + &target + "/" + file;
                                    match remove_file(f) {
                                        Ok(_) => println!("file {} in dir {} deleted", file, &target),
                                        Err(error) => eprintln!("{}", error),
                                    }
                                }
                                match remove_dir(&target) {
                                    Ok(_) => println!("dir {} deleted", &target),
                                    Err(error) => eprintln!("{}", error),
                                }
                            },
                            _ => println!("the removal is canceled"),
                        }
                    },
                }
            },
        }
    }
}