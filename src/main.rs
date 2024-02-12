use clap::Parser;
use memmap2::Mmap;
use rayon::prelude::*;
use std::fs::File;
use xxhash_rust::xxh64::Xxh64;

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

    let vec: Vec<u64> = mmap
        .par_chunks(block_size_bytes)
        .map(|slice| {
            let mut hash = Xxh64::new(0);
            hash.update(slice);
            hash.digest()
        })
        .collect();

    let mut hash = Xxh64::new(0);
    for digest in vec.iter() {
        hash.update(&digest.to_be_bytes());
    }
    println!("{:#x}", hash.digest());

    Ok(())
}
