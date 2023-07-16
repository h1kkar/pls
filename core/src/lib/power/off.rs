pub fn poweroff() {
    match system_shutdown::shutdown() {
        Ok(_) => println!("bye-bye"),
        Err(error) => eprintln!("{}", error),
    }
}