use ring::digest::{Context, Digest, SHA256};
use std::fs::File;
use std::io::{BufReader, Error, Read};
use std::{io::BufRead, path::Path};
use walkdir::WalkDir;

// filepath is of type P and P is a type that must implement the AsRef trait
fn compute_digest<P: AsRef<Path>>(filepath: P) -> Result<Digest, Error> {
    // open file using a buffer
    let mut buffer_reader = BufReader::new(File::open(filepath).unwrap());
    // create a standard buffer of 1024
    let mut buffer = [0; 1024];
    // create contex so we can create a digerst using byte by byte data
    let mut ctx = Context::new(&SHA256);

    loop {
        // count would be the number of bytes that were read into the buffer
        // from the buffer_reader. The value of count would be determined
        // by the number of bytes that were available to be read from the buffer_reader.
        // If there are no more bytes to be read, then count would be 0.
        let count = buffer_reader.read(&mut buffer).unwrap();
        if count == 0 {
            break;
        }
        ctx.update(&buffer[..count]);
    }
    Ok(ctx.finish())
}

fn main() {
    let folder_to_walk = "Midjourney";
    // walk through the directory and compute digest
    for entry in WalkDir::new(&folder_to_walk).min_depth(1) {
        let digest = compute_digest(entry.unwrap().path()).unwrap();
        println!("{:?}", digest);
    }
}
