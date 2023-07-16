pub fn suspend() {
    match system_shutdown::sleep() {
        Ok(_) => println!("sleeping"),
        Err(error) => eprintln!("{}", error),
    }
}