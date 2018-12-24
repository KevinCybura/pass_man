#[macro_use]
extern crate log;
extern crate clap;
extern crate env_logger;

mod creds;
mod sections;

use clap::{App, AppSettings, Arg, ArgMatches};
use std::io;
use std::io::prelude::Write;
use std::path::Path;

fn main() {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let args = parse_arg();
    let mut section = sections::Sections::new();
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
                        let user = sections::new_user();
                        let mut file = sections::initialize_file();
                        file.write_fmt(format_args!("password: {}", user.password))
                            .unwrap();
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
            let user = sections::new_user();
            let mut file = sections::initialize_file();
            file.write_fmt(format_args!("password: {}", user.password))
                .unwrap();
        }
    }
    if args.is_present("site") {
        creds::authenicate_user(&mut section);
        if section.creds.loggedin {
            // section.new_site(args.values_of("site"))
            match args.value_of("site") {
                Some(s) => {
                    section.new_site(&s).unwrap();
                }
                None => {
                    eprint!("Site name is required");
                    warn!("Missing site");
                }
            }

            match section.write_sections() {
                Ok(_) => {
                    info!("Write sections successful");
                }
                Err(e) => {
                    eprintln!("Error with writing sections: {:?}", e);
                    warn!("Error with writing sections: {:?}", e);
                }
            }
            // println!("{:?}", args.value_of("site"));
        }
    }
    if args.is_present("get") {
        creds::authenicate_user(&mut section);
        let section = match sections::parse_file() {
            Ok(section) => {
                info!("File loaded into sections struct: {:?}", section);
                section
            }
            Err(e) => {
                error!("Failed to load file: {:?}", e);
                std::process::exit(1);
            }
        };
        println!("{:?}", section.sites);
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
        .arg(
            Arg::with_name("get")
                .short("g")
                .long("get")
                .takes_value(true),
        )
        .get_matches()
}
