use std::fs::File;
use std::io;
use std::io::prelude::*;
pub struct Cred {
    pub password: String,
    loggedin: bool,
}

impl Cred {
    pub fn new() -> Cred {
        Cred {
            password: String::new(),
            loggedin: false,
        }
    }

    pub fn login(&mut self) -> bool {
        info!("login");
        let mut file_creds = get_creds().unwrap();
        print!("Enter password: ");
        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut self.password) {
            Ok(_) => {
                // When using stdin when the user press enter a \n is also added.
                match self.password.rfind('\n') {
                    Some(n) => {
                        debug!("found \\n pos: {}", n);
                        self.password.remove(n);
                    }
                    None => {}
                };
            }
            Err(e) => {
                error!("Something went wrong with input: {}", e);
            }
        }
        match file_creds.iter().find(|&word| {
            debug!("File pass: {:?}, Entered pass: {:?}", *word, self.password);
            *word == self.password
        }) {
            Some(password) => {
                debug!("Match password: {}", password);
                self.loggedin = true;
            }
            None => {
                error!("Incorrect password");
                self.loggedin = false;

            }
        }
        self.loggedin
    }
}
pub fn new_user() {
    let mut cred = Cred::new();
    println!("Enter new password: ");
    match io::stdin().read_line(&mut cred.password) {
        Ok(_) => {}
        Err(e) => {
            println!("Error {}", e);
        }
    }
    let mut file = initialize_file();
    file.write_fmt(format_args!("password: {}", cred.password))
        .unwrap();
}

pub fn initialize_file() -> File {
    return File::create("creds.txt").unwrap();
}

pub fn get_creds() -> Result<Vec<String>, io::Error> {
    let mut file = match File::open("creds.txt") {
        Ok(file) => {
            info!("Opening file:");
            file
        }
        Err(e) => {
            warn!("Unable to open file: {}", e);
            return Err(e);
        }
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let file_creds = parse_file_creds(contents);
    Ok(file_creds)
}

fn parse_file_creds(contents: String) -> Vec<String> {
    let mut file_creds: Vec<String> = Vec::new();
    for line in contents.lines() {
        let words: Vec<&str> = line.split(":").collect();
        for i in 0..words.len() {
            if words[i].trim() == "password" {
                file_creds.push(String::from(words[i].trim()));
                file_creds.push(String::from(words[i + 1].trim()));
                debug!("Filecreds: {:?}", file_creds);
                return file_creds;
            }
        }
    }
    error!("File creds empty: {:?}", file_creds);
    file_creds
}
