const VERSION: &str = "1.2.0";

pub mod lib {
    pub mod main {
        use std::env::args;
        use crate::lib::{
            hlp::usage,
            opn::{
                self,
                ch,
            },
            files::{
                rm::delete,
                mv::rename,
                cp::copy,
            },
            calc::calculator,
            power::{
                off,
                rb,
                sleep,
                out,
            },
        };

        pub fn start() {
            // pls arguments
            let args: Vec<String> = args().collect();

            match args.len()-1 {
                0 => {
                    println!("no arg received");
                    println!("read \"pls help\" for help");
                },
                
                // default
                _ => {
                    match &args[1][..] {
                        // usage func
                        "help" | "hlp" | "usage" | "-h" => {
                            if args.len()-1 > 1 {
                                more_arg()
                            } else {
                                usage()
                            }
                        },

                        // open shell config
                        "shell" | "sh" => {
                            if args.len()-1 > 1 {
                                more_arg()
                            } else {
                                ch::sh_choise()
                            }
                        },

                        // open window manager config
                        "wm" | "de" => {
                            if args.len()-1 > 1 {
                                more_arg()
                            } else {
                                ch::wm_choise()
                            }
                        }

                        "terminal" | "term" | "tty" => {
                            if args.len()-1 > 1 {
                                more_arg()
                            } else {
                                ch::term_choise()
                            }
                        }
                        // open file
                        "open" | "opn" | "-o" => {
                            if args.len()-1 > 2 {
                                more_arg()
                            } else{
                                println!("{} open {}", ch::editor() ,&args[2]);
                                opn::open(&args[2])
                            }    
                        },

                        // delete files and dirs
                        "remove" | "rm" | "delete" | "del" | "-d" => delete(&args[2..]),

                        // rename files and dirs
                        "rename" | "move" | "mv" | "-r" => rename(&args[2], &args[3]),

                        // copy files and dirs
                        "copy" | "cp" | "-c" => copy(&args[2], &args[3]),

                        // calc
                        "calc" => {
                            match args.len()-1 {
                                1 => calculator(&String::from("")),
                                2 => calculator(&args[2]),
                                _ => more_arg(),
                            }
                        },
                        //calculator(),

                        // poweroff
                        "poweroff" | "off"  => off::poweroff(),

                        // reboot
                        "reboot" |"reb" | "rb" => rb::reboot(),

                        // logout
                        "logout" | "out" => out::logout(),

                        // sleep
                        "suspend" | "sus" | "sleep" => sleep::suspend(),

                        // version
                        "ver" | "-v" => {
                            if args.len()-1 > 1 {
                                more_arg()
                            } else {
                                println!("pls v{}", crate::VERSION)
                            }
                        },

                        // default
                        _ => not_arg(1),
                    }
                },
            }
        }

        fn not_arg(num: usize) {
            let args: Vec<String> = args().collect();
            println!("{} is not pls arg", args[num]);
        }
        fn more_arg() {
            println!("more arg")
        }
    }

    mod hlp;

    mod opn;
    mod files {
        pub mod rm;
        pub mod mv;
        pub mod cp;
    }
    mod calc;
    mod power {
        pub mod off;
        pub mod rb;
        pub mod sleep;
        pub mod out;
    }
}