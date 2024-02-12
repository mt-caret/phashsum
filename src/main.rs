use clap::Parser;
use memmap2::Mmap;
use rayon::prelude::*;
use std::fs::File;
use xxhash_rust::xxh3::Xxh3;

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct Args {
    file: String,

    #[arg(short, long, default_value_t = 1024)]
    block_size_bytes: usize,
}

fn main() -> Result<(), std::io::Error> {
    let Args {
        file,
        block_size_bytes,
    } = Args::parse();

    let file = File::open(file)?;

    let mmap = unsafe { Mmap::map(&file)? };

    let vec: Vec<u128> = mmap
        .par_chunks(block_size_bytes)
        .map(|slice| {
            let mut hash = Xxh3::new();
            hash.update(slice);
            hash.digest128()
        })
        .collect();

    let mut hash = Xxh3::new();
    for digest in vec.iter() {
        hash.update(&digest.to_be_bytes());
    }
    println!("{:#x}", hash.digest128());

    Ok(())
}
