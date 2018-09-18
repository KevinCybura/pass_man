
use std::fs::File;
use std::io;
use std::io::prelude::*;
pub struct Cred {
    pub password: String,
}

impl Cred {
    pub fn new() -> Cred {
        Cred {
            password: String::new(),
        }
    }
    // fn def() -> Cred {}
}
pub fn new_user() {
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

pub fn create_file() -> File {
    return File::create("creds.txt").unwrap();
}
pub fn get_creds() -> Result<File, io::Error> {
    let mut file = match File::open("creds.txt") {
        Ok(file) => file,
        Err(_) => return Err(io::Error::new(io::ErrorKind::NotFound, "Not found")),
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    for l in contents.lines() {
        println!("{:?}", l);
    }
    Ok(file)
    // Ok(User::new("", ""))
}
