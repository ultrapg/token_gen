use clap::{Parser, ValueEnum};
use rand::{RngCore, thread_rng};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use base64::Engine;
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Algorithm {
    Hex,
    Base64,
}

#[derive(Parser)]
#[command(name = "TokenGen", about = "generates tokens-/keyfiles with no real strenght limit")]
struct Cli {
    #[arg(short, long, default_value_t = 1024)]
    bits: usize,

    #[arg(short, long, value_enum, default_value = "hex")]
    algo: Algorithm,

    #[arg(short, long)]
    output: Option<PathBuf>,
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    let total_bytes = (cli.bits + 7) / 8;
    let chunk_size = 8192; 
    let mut bytes_processed = 0;

    let pb = ProgressBar::new(total_bytes as u64);
    pb.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .progress_chars("#>-"));

    let mut writer: Box<dyn Write> = if let Some(path) = cli.output {
        Box::new(BufWriter::new(File::create(path)?))
    } else {
        Box::new(BufWriter::new(std::io::stdout()))
    };

    let mut rng = thread_rng();

    while bytes_processed < total_bytes {
        let remaining = total_bytes - bytes_processed;
        let current_chunk_size = std::cmp::min(remaining, chunk_size);
        let mut buffer = vec![0u8; current_chunk_size];
        
        rng.fill_bytes(&mut buffer);

        let encoded = match cli.algo {
            Algorithm::Hex => hex::encode(&buffer),
            Algorithm::Base64 => base64::engine::general_purpose::STANDARD.encode(&buffer),
        };

        writer.write_all(encoded.as_bytes())?;
        
        bytes_processed += current_chunk_size;
        pb.set_position(bytes_processed as u64);
    }

    pb.finish_with_message("Finished!");
    writer.flush()?;

    Ok(())
}
