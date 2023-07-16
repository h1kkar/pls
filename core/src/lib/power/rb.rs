pub fn reboot() {
    match system_shutdown::reboot() {
        Ok(_) => println!("starting reboot"),
        Err(error) => eprintln!("{}", error),
    }
}