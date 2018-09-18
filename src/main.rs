extern crate clap;

mod creds;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use std::io;
use std::path::Path;

// struct User {
//     user: String,
//     pass: String,
// }

// impl User {
//     fn new(user: &str, pass: &str) -> User {
//         User {
//             user: String::from(user),
//             pass: String::from(pass),
//         }
//     }
// }

fn main() {
    let mut creds = creds::Cred::new();
    if Path::new("creds.txt").exists() {
        println!("Enter password: ");
        match io::stdin().read_line(&mut creds.password) {
            Ok(_) => {}
            Err(error) => eprintln!("Error: {}", error),
        }
    } else {
        println!("No credentials found do you want to create a new user? \n (y/n)");
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => match input.to_lowercase().trim() {
                "y" => {
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
    }
    let args = parse_arg();
    match args.subcommand_name() {
        Some("new") => {}
        Some("get") => {}
        _ => {
            println!("No input add usage is:\n{}", args.usage());
        }
    }
}

fn parse_arg<'a>() -> ArgMatches<'a> {
    App::new("pass-man")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .version("1.0")
        .author("Kevin E. Cybura")
        .about("\nRequired:\nSubcommands are required")
        .subcommand(
            SubCommand::with_name("new")
                .about("enter new site with username and password")
                .arg(
                    Arg::with_name("site")
                        .short("s")
                        .long("site")
                        .takes_value(true)
                        .required(true)
                        .help("the site"),
                ).arg(
                    Arg::with_name("usr")
                        .short("u")
                        .long("usr")
                        .takes_value(true)
                        .required(true)
                        .help("the usrname"),
                ).arg(
                    Arg::with_name("pass")
                        .short("p")
                        .long("pass")
                        .takes_value(true)
                        .required(true)
                        .help("the password"),
                ),
        ).subcommand(
            SubCommand::with_name("get").about("gets pass of site").arg(
                Arg::with_name("site")
                    .short("s")
                    .long("site")
                    .help("the site you want your password for")
                    .required(false)
                    .takes_value(true),
            ),
        ).get_matches()
}
