use creds::Cred;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;

pub struct Site {
    site: String,
    username: String,
    password: String,
}

impl Site {
    pub fn new() -> Site {
        Site {
            site: String::new(),
            username: String::new(),
            password: String::new(),
        }
    }
}

#[derive(Debug)]
pub enum SectionsError {
    InvalidCredentials,
    FsError(std::io::Error),
    MissingFile,
    InputError(std::io::Error),
}
impl From<std::io::Error> for SectionsError {
    fn from(e: std::io::Error) -> SectionsError {
        SectionsError::FsError(e)
    }
}
pub struct Sections {
    pub creds: Cred,
    pub sites: Vec<Site>,
}
impl Sections {
    pub fn new() -> Sections {
        Sections {
            creds: Cred::new(),
            sites: Vec::new(),
        }
    }
    pub fn write_sections(&self) -> Result<(), SectionsError> {
        let mut file = match OpenOptions::new().read(true).write(true).open("creds.txt") {
            Ok(file) => {
                info!("Opening file:");
                file
            }
            Err(e) => {
                warn!("Unable to open file: {}", e);
                return Err(SectionsError::FsError(e));
            }
        };
        // let mut file = match File::open("creds.txt") {
        //     Ok(file) => {
        //         info!("Opening file:");
        //         file
        //     }
        //     Err(e) => {
        //         warn!("Unable to open file: {}", e);
        //         return Err(SectionsError::FsError(e));
        //     }
        // };
        let mut buf = String::new();
        buf.push_str(format!("Creds\npassword:{}\n", self.creds.password).as_str());
        if !self.sites.is_empty() {
            buf.push_str("Sites\n");
            for site in &self.sites {
                buf.push_str(
                    format!(
                        "site:{},username:{},password:{}\n",
                        site.site, site.username, site.password
                    )
                    .as_str(),
                );
            }
        }
        file.write_all(buf.as_bytes())?;
        Ok(())
    }
    pub fn new_site<'a>(&mut self, site: &'a str) -> Result<(), SectionsError> {
        info!("Attempting to create new site");
        println!("Creating new site {}.", site);
        let mut s = Site::new();
        s.site.push_str(site);
        println!("Enter username for {}", site);
        match io::stdin().read_line(&mut s.username) {
            Ok(_) => {
                s.username.retain(|c| c != '\n');
                info!("Successfully read username: {}", s.username);
            }
            Err(e) => {
                eprintln!("{}", e);
                warn!("{}", e);
                return Err(SectionsError::InputError(e));
            }
        }
        println!("Enter password for {}", site);
        match io::stdin().read_line(&mut s.password) {
            Ok(_) => {
                s.password.retain(|c| c != '\n');
                info!("Successfully read password: {}", s.password);
            }
            Err(e) => {
                eprintln!("{}", e);
                warn!("{}", e);
                return Err(SectionsError::InputError(e));
            }
        }
        self.sites.push(s);
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

pub fn initialize_file() -> File {
    File::create("creds.txt").unwrap()
}

pub fn get_creds() -> Result<Vec<String>, SectionsError> {
    let mut file = match File::open("creds.txt") {
        Ok(file) => {
            info!("Opening file:");
            file
        }
        Err(e) => {
            warn!("Unable to open file: {}", e);
            return Err(SectionsError::FsError(e));
        }
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let file_creds = parse_file_creds(&contents);
    file_creds
}

fn parse_file_creds(contents: &str) -> Result<Vec<String>, SectionsError> {
    let mut file_creds: Vec<String> = Vec::new();
    for line in contents.lines() {
        let words: Vec<&str> = line.split(':').collect();
        for mut i in 0..words.len() {
            if words[i].trim() == "password" {
                file_creds.push(String::from(words[i].trim()));
                file_creds.push(String::from(words[i + 1].trim()));
                debug!("Filecreds: {:?}", file_creds);
                return Ok(file_creds);
            }
        }
    }
    error!("File creds empty: {:?}", file_creds);
    Err(SectionsError::MissingFile)
}
