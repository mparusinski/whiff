use clap::CommandFactory;
use clap::Parser;
use std::fs::{File, FileTimes};
use std::io;
use std::path::Path;
use std::process;
use std::time::SystemTime;

#[cfg(feature = "completions")]
use clap_complete::{generate, Shell};

#[derive(Parser, Debug)]
#[command(
    name = "whiff",
    version,
    about = "whiff: a rust replacement for `touch`", long_about = None)]
struct Cli {
    #[arg(short, help = "change only the access time")]
    access: bool,

    #[arg(short = 'c', long, help = "do not create any files")]
    no_create: bool,

    // TODO: Not implemented yet
    #[arg(
        short,
        long,
        value_name = "STRING",
        help = "parse STRING and use it instead of current time"
    )]
    date: Option<String>,

    #[arg(short = 'f', help = "(ignored)")]
    ignore_force: bool,

    // TODO: Not implemented yet
    #[arg(
        short = 'n',
        long,
        help = "affect each symbolic link instead of any referenced file \
                (useful only on systems that can change the timestamps of \
                a symlink)"
    )]
    no_dereference: bool,

    #[arg(short, help = "change only the modification time")]
    modification: bool,

    // TODO: Not implemented yet
    #[arg(
        short,
        long,
        value_name = "FILE",
        help = "use this file's times instead of current time"
    )]
    reference: Option<String>,

    // TODO: Not implemented yet
    #[arg(
        short = 't',
        help = "[[CC]YYMMDDhhmm[.ss] \
                         use specified time instead of current time, with a \
                         date-time format that differs from -d's"
    )]
    specified_time: Option<String>,

    // TODO: Not implemented yet
    #[arg(
        long = "time",
        value_name = "WORD",
        help = "specify which time to change: access (-a): 'access', \
                'atime', 'use'; modification time (-m): 'modify', 'mtime'"
    )]
    time_to_change: Option<String>,

    #[arg(value_name = "FILES")]
    inputs: Vec<String>,

    #[cfg(feature = "completions")]
    #[arg(long, hide = true, exclusive = true)]
    gen_completions: Option<Option<Shell>>,
}

fn whiff(cli: &Cli, path: &String) -> io::Result<()> {
    let filepath = Path::new(&path);
    let fh = if filepath.exists() {
        File::options().append(true).open(path)?
    } else if !cli.no_create {
        // We need to distinguish if path parent exists or not
        match filepath.parent() {
            Some(parent) => {
                if parent != Path::new("") && !parent.exists() {
                    panic!("whiff: cannot whiff '{}': No such file or directory", path)
                }
            }
            None => {} // ignore
        }
        File::create_new(path).expect(format!("Unable to create path {}", path).as_str())
    } else {
        // touch command silently does nothing when
        // file does not exist and option -c is on
        return Ok(());
    };

    let modify_time: SystemTime = cli
        .reference
        .clone()
        .map_or(Ok(SystemTime::now()), |ref_path: String| {
            File::open(ref_path)?.metadata()?.modified()
        })?;
    let access_time: SystemTime = SystemTime::now();

    let times = if cli.access && !cli.modification {
        FileTimes::new().set_accessed(access_time)
    } else if !cli.access && cli.modification {
        FileTimes::new().set_modified(modify_time)
    } else {
        // both set or neither set
        FileTimes::new()
            .set_accessed(access_time)
            .set_modified(modify_time)
    };

    fh.set_times(times)
}
fn input_validation(cli: &Cli) {
    // Input validation
    if cli.inputs.len() == 0 {
        eprintln!("Missing file operand\nTry `whiff --help` for more information");
        process::exit(1);
    }
}

fn main() {
    let cli = Cli::parse();

    // TODO: Refactor this code by moving to another function
    if let Some(maybe_shell) = &cli.gen_completions {
        match maybe_shell {
            Some(sh) => {
                generate(*sh, &mut Cli::command(), "whiff", &mut io::stdout());
            }
            None => {
                eprintln!("Generate completions request but no shell found");
                process::exit(255);
            }
        }
    }

    input_validation(&cli);

    // TODO: Avoid unnecessary clone here under
    let _results: io::Result<()> = cli
        .inputs
        .clone()
        .into_iter()
        .try_for_each(|path| whiff(&cli, &path));
    // TODO: Return error code
}
