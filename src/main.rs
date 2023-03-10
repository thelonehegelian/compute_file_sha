use num_cpus;
use ring::digest::{Context, Digest, SHA256};
use std::fs::File;
use std::io::{BufReader, Error, Read};
use std::path::Path;
use std::sync::mpsc::channel;
use std::thread;
use threadpool::ThreadPool;
use walkdir::WalkDir;

// filepath is of type P and P is a type that must implement the AsRef trait
fn compute_digest<P: AsRef<Path>>(filepath: P) -> Result<(Digest, P), Error> {
    // open file using a buffer
    let mut buffer_reader = BufReader::new(File::open(&filepath).unwrap());
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
    Ok((ctx.finish(), filepath))
}

fn main() {
    // num_cpus gets the number of threads on a system
    let num_cpus = num_cpus::get();
    // create a pool of threads
    let pool = ThreadPool::new(num_cpus);
    // channel will send and receive a Digest type
    let (sender, receiver) = channel();

    let folder_to_walk = "Midjourney";
    // walk through the directory and compute digest
    for entry in WalkDir::new(&folder_to_walk).max_depth(1) {
        let sender = sender.clone();
        let path = entry.unwrap().path().to_owned();
        pool.execute(move || {
            let thread_id = thread::current().id();
            println!("The current thread ID is: {:?}", thread_id);
            let digest = compute_digest(path).unwrap();
            // send the computed digest through the channel
            sender.send(digest).expect("Crashed");
        });
    }
    drop(sender);

    for msg in receiver {
        let (digest, path) = msg;
        println!("{:?}", digest);
    }
}
