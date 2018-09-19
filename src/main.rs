#[macro_use]
extern crate log;
extern crate clap;
extern crate env_logger;

mod creds;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use std::io;
use std::path::Path;

fn main() {
    env_logger::init();
    let args = parse_arg();
    let mut creds = creds::Cred::new();
    match args.subcommand_name() {
        Some("new") => {
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
        Some("get") => {}
        Some("site") => {
            if !creds.login() {
                return;
            }
        }
        _ => {}
    }
}

fn parse_arg<'a>() -> ArgMatches<'a> {
    App::new("pass-man")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .version("1.0")
        .author("Kevin E. Cybura")
        .about("\nRequired:\nSubcommands are required")
        .subcommand(SubCommand::with_name("new").about("enter new site with username and password"))
        .subcommand(
            SubCommand::with_name("get").about("gets pass of site").arg(
                Arg::with_name("site")
                    .short("s")
                    .long("site")
                    .help("the site you want your password for")
                    .takes_value(true),
            ),
        ).subcommand(
            SubCommand::with_name("site")
                .about("adds a new site")
                .arg(
                    Arg::with_name("site")
                        .short("s")
                        .long("site")
                        .takes_value(true)
                        .required(true)
                        .help("the site"),
                )//.arg(
                //     Arg::with_name("usr")
                //         .short("u")
                //         .long("usr")
                //         .takes_value(true)
                //         .required(true)
                //         .help("the usrname"),
                //)//.arg(
                    // Arg::with_name("pass")
                    //     .short("p")
                    //     .long("pass")
                    //     .takes_value(true)
                    //     .required(true)
                    //     .help("the password"),
                // ),
        ).get_matches()
}
