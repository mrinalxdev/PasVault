use serde::{Deserialize, Serialize};
use std::fs::File;
use std::fs::OpenOptions; 
use std::io;
use std:io::BufRead;
use std::io::Write;



#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceInfo{
    pub service: String,
    pub username: String,
    pub password: String,
}

impl ServiceInfo{
    pub fn new(service: String, username: String, password: String) -> Self{
        ServiceInfo {
            service,
            username, 
            password
        }
    }

    pub fn from_json(json_string: &str) -> Result<Self, serde_json::Error>{
        serde_json::from_str(json_string)
    }

}

fn prompt() {}

fn read_password_from_file() {}
