use sections::{get_creds, Sections, SectionsError};
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Cred {
    pub password: String,
    pub loggedin: bool,
}

impl Cred {
    pub fn new() -> Cred {
        Cred {
            password: String::new(),
            loggedin: false,
        }
    }

    pub fn login(&mut self, file: &[String]) -> Result<(), SectionsError> {
        info!("login");
        print!("Enter password: ");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut self.password) {
            Ok(_) => {
                self.password.retain(|c| c != '\n');
            }
            Err(e) => {
                error!("Something went wrong with input: {}", e);
            }
        }
        file.iter().find(|&word| {
            debug!("File pass: {:?}, Entered pass: {:?}", *word, self.password);
            if &self.password == word {
                self.loggedin = true;
            }
            self.loggedin
        });
        if !self.loggedin {
            error!("Incorrect Password: {}", self.password);
            eprint!("Incorrect Password: {}", self.password);
            return Err(SectionsError::InvalidCredentials);
        }
        Ok(())
    }
}

pub fn new_user() -> Cred {
    let mut cred = Cred::new();
    println!("Enter new password: ");
    match io::stdin().read_line(&mut cred.password) {
        Ok(_) => {}
        Err(e) => {
            println!("Error {}", e);
        }
    }
    cred
}
pub fn authenicate_user(section: &mut Sections) -> bool {
    let file = match get_creds() {
        Ok(file) => file,
        Err(e) => {
            error!("Fatal error when loading file: {:?}", e);
            return false;
        }
    };
    match section.creds.login(&file) {
        Ok(_) => info!("Successful login"),
        Err(e) => {
            eprint!("Invalid credentials: {:?}", e);
            debug!("Invalid credentials: {:?}", e);
            return false;
        }
    }
    true
}
