pub mod lib {
    pub mod main {
        use std::{
            env::args,
            process::Command
        };
        use crate::lib::{
            help::usage,
            opn::{
                self,
                ch,
            },
            rm::delete,
            mv::rename,
            cp::copy,
            calc::calculator,
            power,
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
                        "rm" | "del" | "remove" | "delete" | "-d" => delete(&args[2..]),

                        // rename files and dirs
                        "rename" | "move" | "mv" | "-r" => rename(&args[2], &args[3]),

                        // copy files and dirs
                        "copy" | "cp" | "-c" => copy(&args[2], &args[3]),

                        // calc
                        "calc" => calculator(),

                        // poweroff
                        "poweroff" | "off"  => power::off(),

                        // reboot
                        "reboot" |"reb" | "rb" => power::reboot(),

                        // logout
                        "logout" | "out" => power::out(),

                        // sleep
                        "suspend" | "sus" | "sleep" => power::suspend(),

                        // version
                        "ver" | "-v" => {
                            if args.len()-1 == 2 {
                                match Command::new(&args[2][..])
                                    .arg("--version")
                                    .spawn() {
                                        Ok(_) => return,
                                        Err(error) => eprintln!("{}" ,error),
                                    }
                            } else if args.len()-1 > 1 {
                                more_arg()
                            } else {
                                println!("pls v1.0.0")
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

    mod help;

    mod opn;
    mod rm;
    mod mv;
    mod cp;
    mod calc;
    mod power;
}