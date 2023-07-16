pub fn logout() {
    match system_shutdown::logout() {
        Ok(_) => println!("exit"),
        Err(error) => eprintln!("{}", error),
    }
}