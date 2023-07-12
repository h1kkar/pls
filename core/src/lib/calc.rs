use std::io::{
    self,
    prelude::*
};

pub fn calculator() {
    
    //println!("{}", msg.trim());

    loop {
        print!("❯ ");
        io::stdout().flush().ok().expect("err");

        let mut msg: String = String::new();
        let _ = io::stdin().read_line(&mut msg);
        let msg = String::from(msg.trim());

        for i in vec![String::from("exit"), String::from("quit"), String::from("q"), String::from(":q")] {
            if msg == i {
                println!("bye-bye");
                return;
            }
        }

        let (pos, act) = pos(&msg);

        if pos == 0 {
            println!("error")
        } else {
            println!("= {}\n", calc(&msg, pos, act));
        }
    }
}

fn pos<'a>(msg: &String) -> (usize, &str) {
    let bytes = msg.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
            /*match item {
                b'+' | b'-' | b'*' | b'/' => {
                    return i;
                },
                _ => return i,
            }*/
            if item == b'+' {
                return (i+1, "+");
            } else if item == b'-' {
                return (i+1, "-");
            } else if item == b'*' {
                return (i+1, "*");
            } else if item == b'/' {
                return (i+1, "/");
            };
        }
    return (0, "");
}

fn calc(msg: &String, pos: usize, act: &str) -> usize {
    let first = match msg[..pos-1].parse() {
        Ok(n) => n,
        Err(_) => {
            0
        },
    } as usize;
    let second = match msg[pos..].parse() {
        Ok(n) => n,
        Err(_) => {
            0
        },
    } as usize;

    match act {
        "+" => first+second,
        "-" => first-second,
        "*" => first*second,
        "/" => first/second,
        _ => 0,
    }
}