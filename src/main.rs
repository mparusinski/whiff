use std::env;
use std::fs::File;
use std::io;
use std::option;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, help = "change only the access time")]
    access: bool,

    #[arg(short = 'c', long, help = "do not create any files")]
    no_create: bool,

    #[arg(short, long, value_name = "STRING",
        help = "parse STRING and use it instead of current time")]
    date: Option<String>,

    #[arg(short = 'f', help = "(ignored)")]
    ignore_force: bool,

    #[arg(short = 'n', long,
        help = "affect each symbolic link instead of any referenced file \
                (useful only on systems that can change the timestamps of \
                a symlink)")]
    no_dereference: bool,

    #[arg(short, help = "change only the modification time")]
    modification: bool,

    #[arg(short, long, value_name = "FILE", help = "use this file's times instead of current time")]
    reference: Option<String>,

    #[arg(short = 't', help = "[[CC]YYMMDDhhmm[.ss] \
                         use specified time instead of current time, with a \
                         date-time format that differs from -d's")]
    specified_time: Option<String>,

    #[arg(long = "time", value_name = "WORD",
        help = "specify which time to change: access (-a): 'access', \
                'atime', 'use'; modification time (-m): 'modify', 'mtime'")]
    time_to_change: Option<String>,
}

fn main() {
    let args = Args::parse();
    println!("{args:#?}");

    // let results: io::Result<Vec<File>> = env::args().skip(1).
    //     map(File::create_new).
    //     collect();
    // println!("Results: {:?}", results)
}
