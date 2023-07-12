use std::{
    env,
    //process::Command
};

pub fn var(cmd: &str) -> String {
    let cmd = cmd.to_uppercase();
    match env::var(cmd) {
        Ok(output) => {
            output
        },
        Err(error) => {
            eprintln!("{}", error);
            String::from("")
        }
    }
}