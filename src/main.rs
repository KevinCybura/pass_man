#[macro_use]
extern crate log;
extern crate clap;
extern crate env_logger;

mod creds;

use clap::{App, AppSettings, Arg, ArgMatches};
use std::io;
use std::path::Path;

fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let args = parse_arg();
    let mut creds = creds::Cred::new();
    if args.is_present("new") {
        if Path::new("creds.txt").exists() {
            println!(
                "A store of passwords exists\n\
                 by creating a new user you will \
                 delete this store"
            );
            println!("\nDo you want to delete the store and create a new user? (y/n): ");

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(_) => match input.to_lowercase().trim() {
                    "y" => {
                        // Require the user to enter password here and validate that its corret
                        std::fs::remove_file("creds.txt").unwrap();
                        creds::new_user();
                    }
                    "n" => {
                        std::process::exit(1);
                    }
                    err => {
                        println!("Invalid input: {}", err);
                        std::process::exit(1);
                    }
                },
                Err(error) => eprintln!("Error: {}", error),
            }
        } else {
            creds::new_user();
        }
    }
    if args.is_present("site") {
        if creds.login() {}
        std::process::exit(1);
    }
}

fn parse_arg<'a>() -> ArgMatches<'a> {
    App::new("pass-man")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version("1.0")
        .author("Kevin E. Cybura")
        .about("\nRequired:\nSubcommands are required")
        .arg(
            Arg::with_name("site")
                .short("s")
                .long("site")
                .takes_value(true),
        )
        .arg(Arg::with_name("new").short("n").long("new"))
        .get_matches()
}
