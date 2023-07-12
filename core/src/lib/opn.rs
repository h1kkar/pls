use std::process::Command;

mod run;
pub mod ch;

pub fn open(file: &str) {
    let editor = ch::editor();
    let cmd = editor + " " + file;

    Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .spawn()
        .expect("err")
        .wait()
        .expect("err");
}

pub fn sh() -> String {
    run::var("shell")
}

pub fn wm() -> String {
    run::var("desktop_session")
}

pub fn tty() -> String {
    run::var("term")
}