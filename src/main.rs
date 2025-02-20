use std::fs;
use std::io::{self, Write};
use std::path::Path;
use reqwest::blocking::get;
use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Download {
    url: String,
    filename: String,
}

fn main() {
    let download = Download {
        url: String::from("https://example.com/file.zip"),
        filename: String::from("file.zip"),
    };
    if let Err(e) = download_file(&download) {
        eprintln!("Error downloading file: {}", e);
    }
}

fn download_file(download: &Download) -> Result<(), Box<dyn std::error::Error>> {
    let response = get(&download.url)?;
    let mut file = fs::File::create(&download.filename)?;
    let content = response.bytes()?;
    file.write_all(&content)?;
    println!("Downloaded: {}", download.filename);
    Ok(())
}