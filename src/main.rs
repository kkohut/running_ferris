extern crate reqwest;

use std::time::{Instant, Duration};
use colored::Colorize;

const FILE_SIZE_IN_MB: i32 = 100;

fn main() {
    let start_time = Instant::now();
    perform_download(FILE_SIZE_IN_MB);
    let duration = start_time.elapsed();
    println!("Download finished in {duration:.3?}");
    let speed = calculate_speed(FILE_SIZE_IN_MB, duration);
    println!("Download speed: {}", format!("{speed:.3}Mb/s").green().bold());
}

fn perform_download(file_size: i32) {
    const BASE_URL: &str = "http://speedtest.tele2.net";
    let url = format!("{BASE_URL}/{file_size}MB.zip");
    println!("Downloading {FILE_SIZE_IN_MB}MB from {url}...");
    let _ = reqwest::blocking::get(url).unwrap().bytes();
}

fn calculate_speed(file_size_in_mb: i32, duration: Duration) -> f64 {
    const BITS_PER_BYTE: i32 = 8;
    (file_size_in_mb * BITS_PER_BYTE) as f64 / duration.as_secs() as f64
}