use std::io;
use std::io::prelude::*;
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

    pub fn login(&mut self, file: Vec<String>) {
        info!("login");
        print!("Enter password: ");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut self.password) {
            Ok(_) => {
                if let Some(n) = self.password.rfind('\n') {
                    debug!("found \\n pos: {}", n);
                    self.password.remove(n);
                }
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
            info!("Incorrect Password: {}", self.password);
            eprint!("Incorrect Password: {}", self.password)
        }
    }
}
