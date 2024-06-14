use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::env;

fn generate_random_line(rng: &mut StdRng, counter: u64, max_size: usize) -> String {
    let base_length = max_size - counter.to_string().len() - 1;
    let line_length = rng.gen_range(1..=base_length);
    let line: String = rng
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(line_length)
        .map(char::from)
        .collect();
    format!("{}{}\n", line, counter)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        eprintln!("Usage: {} <seed> <max_line_size> <total_size_gb> <file_path>", args[0]);
        return;
    }

    let seed: u64 = args[1].parse().expect("Invalid seed");
    let max_line_size: usize = args[2].parse().expect("Invalid max line size");
    let total_size_gb: usize = args[3].parse().expect("Invalid total size in GB");
    let file_path = &args[4];

    let total_size_bytes = total_size_gb * 1024 * 1024 * 1024;

    let mut rng = StdRng::seed_from_u64(seed);
    let offset1 = rng.gen_range(0..total_size_bytes);
    let offset2 = rng.gen_range(0..total_size_bytes);
    let (offset1, offset2) = if offset1 < offset2 { (offset1, offset2) } else { (offset2, offset1) };

    let mut file = BufWriter::new(File::create(file_path).expect("Unable to create file"));
    let mut generated_size = 0;
    let mut counter = 0;
    let mut recorded_line: Option<String> = None;
    let mut inserted = false;

    while generated_size < total_size_bytes {
        let line = generate_random_line(&mut rng, counter, max_line_size);
        let line_bytes = line.as_bytes();

        if !inserted && generated_size >= offset2 {
            if let Some(ref rec_line) = recorded_line {
                file.write_all(rec_line.as_bytes()).expect("Unable to write to file");
                generated_size += rec_line.len();
                inserted = true;
                println!("Inserted recorded line at offset {}: {}", offset2, rec_line.trim());
            }
        }

        file.write_all(line_bytes).expect("Unable to write to file");
        generated_size += line_bytes.len();

        if generated_size >= offset1 && recorded_line.is_none() {
            recorded_line = Some(line.clone());
            println!("Recorded line at offset {}: {}", offset1, line.trim());
        }

        counter += 1;
    }

    println!("Generated text file: {}, Size: {} GB", file_path, total_size_gb);
    if let Some(ref rec_line) = recorded_line {
        println!("Recorded line data: {}", rec_line.trim());
    }
}
