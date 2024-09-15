use std::fs::{File, FileTimes};
use std::time::SystemTime;
use std::io;
use std::path::Path;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version,
    about = "whiff: a rust replacement for `touch`", long_about = None)]
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

    #[arg(value_name = "FILES")]
    inputs: Vec<String>
}

fn whiff(args: &Args, path: String) -> io::Result<()>{
    let fh = if Path::new(&path).exists() {
        File::options().append(true).open(path)?
    } else {
        File::create_new(path)?
    };
    let now = SystemTime::now();
    let times = FileTimes::new()
        .set_accessed(now)
        .set_modified(now);
    fh.set_times(times)
}

fn main() {
    let args = Args::parse();
    // println!("{args:#?}");

    let results: io::Result<()> = args.inputs.clone().into_iter()
        .map(|path| {
            whiff(&args, path)
        })
        .collect();
    // println!("Results: {:?}", results)
}
