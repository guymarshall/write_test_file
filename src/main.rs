use promptput::input;
use rand::{distributions::Alphanumeric, Rng};
use std::fs::File;
use std::io::{self, Write};
use std::time::Instant;

fn generate_random_data(size_gb: u64) -> io::Result<()> {
    let billion_in_bytes: u64 = 1024 * 1024 * 1024;
    let size_bytes: u64 = size_gb * billion_in_bytes;

    let random_data: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(1024)
        .map(char::from)
        .collect();

    let random_data_bytes: &[u8] = random_data.as_bytes();
    let random_data_length: u64 = random_data_bytes.len() as u64;

    let mut file: File = File::create("random_data.txt")?;
    let mut bytes_written: u64 = 0;

    while bytes_written < size_bytes {
        file.write_all(random_data_bytes)?;
        bytes_written += random_data_length;

        if bytes_written % billion_in_bytes == 0 {
            println!("{} GB written", bytes_written / billion_in_bytes);
        }
    }

    Ok(())
}

fn main() {
    let size_gb: u64 = input("Number of gigabytes to generate: ");

    let start_time: Instant = Instant::now();

    if let Err(e) = generate_random_data(size_gb) {
        eprintln!("Error generating random data: {}", e);
    }

    let end_time: Instant = Instant::now();

    println!("Elapsed time: {:?}", end_time - start_time);
}
