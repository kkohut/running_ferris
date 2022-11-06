extern crate reqwest;

use std::time::Instant;

const FILE_SIZE_IN_MB: i32 = 100;

fn main() {
    let start_time = Instant::now();
    perform_download(FILE_SIZE_IN_MB).expect("Error while downloading.");
    let duration = start_time.elapsed();
    println!("Downloaded {FILE_SIZE_IN_MB}MB in {duration:?}");
    const BITS_PER_BYTE: i32 = 8;
    let speed = (FILE_SIZE_IN_MB * BITS_PER_BYTE) as f64 / duration.as_secs() as f64;
    println!("Speed: {speed}MB/s");
}

fn perform_download(file_size: i32) -> Result<(), Box<dyn std::error::Error>> {
    const BASE_URL: &str = "http://speedtest.tele2.net";
    let url = format!("{BASE_URL}/{file_size}MB.zip");
    println!("Downloading {FILE_SIZE_IN_MB}MB from {url}...");
    let _ = reqwest::blocking::get(url)?.bytes();
    Ok(())
}