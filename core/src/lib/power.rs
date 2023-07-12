use system_shutdown::{
    shutdown,
    logout,
    sleep
};

pub fn off() {
    match shutdown() {
        Ok(_) => println!("bye-bye"),
        Err(error) => eprintln!("{}", error),
    }
}

pub fn reboot() {
    match system_shutdown::reboot() {
        Ok(_) => println!("starting reboot"),
        Err(error) => eprintln!("{}", error),
    }
}

pub fn out() {
    match logout() {
        Ok(_) => println!("exit"),
        Err(error) => eprintln!("{}", error),
    }
}

pub fn suspend() {
    match sleep() {
        Ok(_) => println!("sleeping"),
        Err(error) => eprintln!("{}", error),
    }
}