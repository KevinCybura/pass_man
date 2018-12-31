#[macro_use]
extern crate log;
extern crate clap;
extern crate env_logger;

mod creds;
mod sections;

use clap::{App, AppSettings, Arg, ArgMatches};
use sections::Site;
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
                        let user = creds::new_user();
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
            let user = creds::new_user();
            let mut file = sections::initialize_file();
            file.write_fmt(format_args!("password: {}", user.password))
                .unwrap();
        }
    }
    creds::authenicate_user(&mut section);

    match sections::parse_file(&mut section) {
        Ok(section) => {
            info!("File loaded into sections struct: {:?}", section);
            section
        }
        Err(e) => {
            error!("Failed to load file: {:?}", e);
            std::process::exit(1);
        }
    };
    if args.is_present("site") && section.creds.loggedin {
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
    }
    if args.is_present("get") {
        match args.value_of("get") {
            Some(s) => {
                let mut site = Site::new();
                site.site = s.to_string();
                match section.sites.get(&site) {
                    Some(s) => {
                        println!(
                            "site: {}, username: {}, password: {}",
                            s.site, s.username, s.password
                        );
                    }
                    None => {
                        warn!("Site {} does not exist", site.site);
                        eprintln!("Site {} does not exist", site.site);
                    }
                }
            }
            None => {
                warn!("Site name is required for get");
                eprintln!("Site name is require for get");
            }
        }
    }

    if args.is_present("all") {
        for s in &section.sites {
            println!("section: {:?}", s);
        }
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
        .arg(Arg::with_name("all").short("a").long("all"))
        .get_matches()
}
