use creds::Cred;
use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::hash::{Hash, Hasher};
use std::io;
use std::io::prelude::*;

#[derive(Debug)]
pub struct Site {
    pub site: String,
    pub username: String,
    pub password: String,
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
impl PartialEq for Site {
    fn eq(&self, s: &Site) -> bool {
        self.site.to_lowercase() == s.site.to_lowercase()
    }
}
impl Eq for Site {}

impl Hash for Site {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.site.to_lowercase().hash(state);
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
#[derive(Debug)]
pub struct Sections {
    pub creds: Cred,
    pub sites: HashSet<Site>,
}
impl Sections {
    pub fn new() -> Sections {
        Sections {
            creds: Cred::new(),
            sites: HashSet::new(),
        }
    }
    pub fn write_sections(&self) -> Result<(), SectionsError> {
        let mut file = match OpenOptions::new().read(true).write(true).open("creds.txt") {
            Ok(file) => {
                info!("Opening file:");
                file
            }
            Err(e) => {
                error!("Unable to open file: {}", e);
                return Err(SectionsError::FsError(e));
            }
        };
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
                error!("{}", e);
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
                error!("{}", e);
                return Err(SectionsError::InputError(e));
            }
        }
        if self.sites.contains(&s) {
            warn!("Site already present: {:?}", s);
            println!("Site already present{:?}", s);
        } else {
            self.sites.insert(s);
        }

        Ok(())
    }
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
            error!("Unable to open file: {}", e);
            return Err(SectionsError::FsError(e));
        }
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    parse_file_creds(&contents)
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

pub fn parse_file() -> Result<Sections, SectionsError> {
    let mut file = match File::open("creds.txt") {
        Ok(f) => {
            info!("File successfully opened");
            f
        }
        Err(e) => {
            error!("Unable to open file: {}", e);
            return Err(SectionsError::FsError(e));
        }
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut section = (false, false);
    let mut result = Sections::new();
    for line in contents.lines() {
        let words: Vec<&str> = line.split(',').collect();
        if words.len() == 1 {
            if words[0] == "Creds" {
                section.0 = true;
                section.1 = false;
            } else if words[0] == "Sites" {
                section.0 = false;
                section.1 = true;
            }
        } else {
            let mut s = Site::new();
            if section.0 {
                if line == "password" {
                    result.creds.password.push_str(line);
                }
            } else if section.1 {
                for word in words {
                    let site_creds: Vec<&str> = word.split(':').collect();
                    if site_creds[0] == "site" {
                        s.site.push_str(site_creds[1]);
                    } else if site_creds[0] == "username" {
                        s.username.push_str(site_creds[1]);
                    } else if site_creds[0] == "password" {
                        s.password.push_str(site_creds[1]);
                    } else {
                        warn!(
                            "Unkown Site parameter: {}\n for str: {}",
                            site_creds[0], word
                        );
                    }
                }
                if result.sites.contains(&s) {
                    error!(
                        "Fatal Error: File contains multiple copies of site: {:?}",
                        s
                    );
                    eprintln!(
                        "Fatal Error: File contains multiple copies of site: {:?}",
                        s
                    );
                } else {
                    result.sites.insert(s);
                }
            }
        }
    }
    Ok(result)
}
