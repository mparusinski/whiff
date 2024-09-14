use std::env;
use std::fs::File;
use std::io;

fn main() {
    let results: io::Result<Vec<File>> = env::args().skip(1).
        map(File::create_new).
        collect();
    println!("Results: {:?}", results)
}
