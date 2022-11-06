use std::time::{Duration, Instant};

use colored::Colorize;
use reqwest::blocking::Client;
use text_io::read;

fn main() {
    let chosen_file_size = ask_for_file_size();
    let start_time = Instant::now();
    perform_download(chosen_file_size);
    let duration = start_time.elapsed();
    println!("Download finished in {duration:.3?}");
    let speed = calculate_speed(chosen_file_size, duration);
    println!(
        "Download speed: {}",
        format!("{speed:.3}Mb/s").green().bold()
    );
}

fn ask_for_file_size() -> usize {
    let allowed_file_sizes_in_mb: [usize; 5] = [1, 10, 100, 1_000, 10_000];
    print_input_instructions(allowed_file_sizes_in_mb);
    let mut optional_file_size_in_mb: Option<usize> = None;
    while optional_file_size_in_mb.is_none() {
        let entered_number: usize = read!();
        if (entered_number - 1) >= allowed_file_sizes_in_mb.len() {
            print_input_instructions(allowed_file_sizes_in_mb);
        } else {
            optional_file_size_in_mb = Some(allowed_file_sizes_in_mb[entered_number - 1]);
        }
    }
    optional_file_size_in_mb.expect("Error occured while entering file size.")
}

fn print_input_instructions(allowed_file_sizes_in_mb: [usize; 5]) {
    println!("Choose your preferred filesize by entering the corresponding number in the square brackets. \
    Larger files result in preciser results.");
    for (index, size) in allowed_file_sizes_in_mb.iter().enumerate() {
        println!("[{}] {}MB", index + 1, size);
    }
}

fn perform_download(file_size_in_mb: usize) {
    const BASE_URL: &str = "http://speedtest.tele2.net";
    let path = if file_size_in_mb >= 1000 {
        let file_size_in_gb = file_size_in_mb / 1000;
        format!("{file_size_in_gb}GB.zip")
    } else {
        format!("{file_size_in_mb}MB.zip")
    };
    let url = format!("{BASE_URL}/{path}");
    println!("\nDownloading {file_size_in_mb}MB from {url}...");
    let client = Client::builder()
        .timeout(Duration::new(600, 0))
        .build()
        .expect("Error occured during client configuration.");
    let _ = client
        .get(url)
        .send()
        .expect("Error occured during download.")
        .bytes();
}

fn calculate_speed(file_size_in_mb: usize, duration: Duration) -> f64 {
    const BITS_PER_BYTE: i32 = 8;
    (file_size_in_mb as i32 * BITS_PER_BYTE) as f64 / duration.as_millis() as f64 * 1_000.0
}
