extern crate clap;
use clap::{App, Arg, ArgMatches, AppSettings, SubCommand};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

struct Cred {
    password: String,
}

impl Cred {
    fn new() -> Cred {
        Cred {
            password: String::new(),
        }
    }
    // fn def() -> Cred {}
}

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
    let mut creds = Cred::new();
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
                    new_user();
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
fn new_user() {
    let mut cred = Cred::new();
    println!("Enter password: ");
    match io::stdin().read_line(&mut cred.password) {
        Ok(_) => {}
        Err(e) => {
            println!("Error {}", e);
        }
    }
    let mut file = create_file();
    file.write_fmt(format_args!("password: {}", cred.password))
        .unwrap();
}

fn create_file() -> File {
    return File::create("creds.txt").unwrap();
}

fn get_creds() -> Result<File, std::io::Error> {
    let mut file = match File::open("creds.txt") {
        Ok(file) => file,
        Err(_) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Not found",
            ))
        }
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    for l in contents.lines() {
        println!("{:?}", l);
    }
    Ok(file)
    // Ok(User::new("", ""))
}
