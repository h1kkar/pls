use std::process::Command;
use super::{super::opn::{
    self,
    open,
    run,
}, run::var};

pub fn sh_choise() {
    let sh = opn::sh();
    let sh: Vec<&str> = sh.split('/').collect();

    match sh[sh.len()-1] {
        // open bash config
        "bash" => {
            let config = home(".bashrc");
            let file: Vec<&str> = config.split('/').collect();
            println!("{} open {}", editor(), file[file.len()-1]);
            open(&config)
        },

        // open hilbish config
        "hilbish" => {
            let config = conf("hilbish/init.lua");
            let file: Vec<&str> = config.split('/').collect();
            println!("{} open {}", editor(), file[file.len()-1]);
            open(&config)
        },

        // open fish config

        "fish" => {
            let config = conf("fish/config.fish");
            let file: Vec<&str> = config.split('/').collect();
            println!("{} open {}", editor(), file[file.len()-1]);
            open(&config)
        },
        
        // open sh config (or not open :)
        "sh" => println!("config not found"),

        // open zsh config
        "zsh" => {
            let config = home(".zshrc");
            let file: Vec<&str> = config.split('/').collect();
            println!("{} open {}", editor(), file[file.len()-1]);
            open(&config)
        },

        // default
        _ => println!("planning, bro"),
    }
}

pub fn wm_choise() {
    match &opn::wm()[..] {
        // open awesomewm config
        "awesome" => {
            let config = conf("awesome/rc.lua");
            let file: Vec<&str> = config.split('/').collect();
            println!("{} open {}", editor(), file[file.len()-1]);
            open(&config)
        },

        // open berry config
        "berry" => {
            let config = conf("berry/autostart");
            let file: Vec<&str> = config.split('/').collect();
            println!("{} open {}", editor(), file[file.len()-1]);
            open(&config)
        },

        // open bspwm config
        "bspwm" => {
            let config = conf("bspwm/bspwmrc");
            let file: Vec<&str> = config.split('/').collect();
            println!("{} open {}", editor(), file[file.len()-1]);
            open(&config)
        },

        // open gnome settings
        "gnome" | "gnome-xorg" => {
            println!("open gnome settings");
            Command::new("sh")
                .arg("-c")
                .arg("gnome-control-center")
                .spawn()
                .expect("err")
                .wait()
                .expect("err");
        },

        // open herbstluftwm config
        "herbstluftwm" => {
            let config = conf("herbstluftwm/autostart");
            let file: Vec<&str> = config.split('/').collect();
            println!("{} open {}", editor(), file[file.len()-1]);
            open(&config)
        },

        // open hyprland config
        "hyprland" => {
            let config = conf("hypr/hyprland.conf");
            let file: Vec<&str> = config.split('/').collect();
            println!("{} open {}", editor(), file[file.len()-1]);
            open(&config)
        },

        // open openbox config
        "openbox" => {
            let config = conf("openbox/rc.ml");
            let file: Vec<&str> = config.split('/').collect();
            println!("{} open {}", editor(), file[file.len()-1]);
            open(&config)
        },

        // default
        _ => println!("planning, bro"),
    }
}

pub fn term_choise() {
        match &opn::tty()[..] {
            "alacritty" => {
                let config = conf("alacritty/alacritty.yml");
                let file: Vec<&str> = config.split('/').collect();
                println!("{} open {}", editor(), file[file.len()-1]);
                open(&config)
            },
            "urxvt" | "xterm" => {
                let config = home(".Xresources");
                let file: Vec<&str> = config.split('/').collect();
                println!("{} open {}", editor(), file[file.len()-1]);
                open(&config)
            },
            "kitty" => {
                let config = conf("kitty/kitty.conf");
                let file: Vec<&str> = config.split('/').collect();
                println!("{} open {}", editor(), file[file.len()-1]);
                open(&config)
            },
            "st" => {
                let config = conf("st/xresources");
                let file: Vec<&str> = config.split('/').collect();
                println!("{} open {}", editor(), file[file.len()-1]);
                open(&config)
            },
            "wezterm" | "wezterm-gui" => {
                let config = conf("wezterm/wezterm.lua");
                let file: Vec<&str> = config.split('/').collect();
                println!("{} open {}", editor(), file[file.len()-1]);
                open(&config)
            },
            _ => println!("planning, bro"),
        }
}

fn home(file: &str) -> String {
    let home = String::from("/home/") + &run::var("user") + "/" + file;
    home
} 

fn conf(file: &str) -> String {
    let conf = String::from("/home/") + &run::var("user") + "/" + ".config/" + file;
    conf
}

pub fn editor() -> String {
    var("editor")
}