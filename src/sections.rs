use creds::Cred;
use std::fs::File;
use std::io;
use std::io::prelude::*;

pub struct Site {
    site: String,
    username: String,
    password: String,
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
    pub fn write_sections(&self) -> Result<(), io::Error> {
        let mut file = match File::open("creds.txt") {
            Ok(file) => {
                info!("Opening file:");
                file
            }
            Err(e) => {
                warn!("Unable to open file: {}", e);
                return Err(e)
            }
        };
        let mut buf = String::new();
        buf.push_str(format!("Creds\npassword:{}\n", self.creds.password).as_str());
        file.write(buf.as_bytes())?;
        if !self.sites.is_empty() {
            buf.clear();
            buf.push_str("Sites\n");
            for site in &self.sites {
                buf.push_str(format!("site:{},username:{},password:{}\n",site.site, site.username, site.password).as_str());
            }
            file.write(buf.as_bytes())?;
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

pub fn initialize_file() -> File {
    File::create("creds.txt").unwrap()
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
    let file_creds = parse_file_creds(&contents);
    Ok(file_creds)
}

fn parse_file_creds(contents: &str) -> Vec<String> {
    let mut file_creds: Vec<String> = Vec::new();
    for line in contents.lines() {
        let words: Vec<&str> = line.split(':').collect();
        for mut i in 0..words.len() {
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
