use flate2::{write::GzEncoder, Compression};
use std::env::args;
use std::fs::File;
use std::io::copy;
use std::io::{self, BufReader};
use std::time::Instant;

fn main() -> io::Result<()> {
    let error_message = "An Error Occurred";
    let args: Vec<String> = args().collect();
    if args.len() != 3 {
        eprintln!("{}: Invalid number of arguments", error_message);
        return Ok(());
    }
    let file_input_path = &args[1];
    let file = File::open(file_input_path)?;
    let mut input = BufReader::new(file);

    let file_output_path = &args[2];
    let output = File::create(file_output_path)?;
    let mut encoder = GzEncoder::new(output, Compression::default());

    let start = Instant::now();
    copy(&mut input, &mut encoder).unwrap();
    let output = encoder.finish().unwrap();
    println!(
        "Source len: {:?}",
        input.get_ref().metadata().unwrap().len()
    );
    println!("Target len: {:?}", output.metadata().unwrap().len());
    println!("Time Elapsed: {:?}", start.elapsed());
    Ok(())
}
