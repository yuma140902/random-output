use clap::Parser;
use crossterm::style::Stylize;
use rand::seq::SliceRandom;
use std::{
    borrow::{Borrow, Cow},
    iter,
};

#[derive(Debug, Parser)]
#[clap(author, version, about = "Output random lines to stdout and stderr")]
struct Args {
    #[clap(long = "n", short = 'n', default_value_t = 10)]
    stdout_lines: usize,

    #[clap(long = "e", short = 'e', default_value_t = 10)]
    stderr_lines: usize,

    #[clap(
        long = "wait",
        short = 'w',
        default_value_t = 0,
        help = "wait millisecond between outputs"
    )]
    wait_ms: u64,

    #[clap(long = "exit", default_value_t = 0)]
    exit_code: i32,

    #[clap(long = "date", short = 'd', help = "show dates")]
    with_dates: bool,

    #[clap(long = "level", short = 'l', help = "show [INFO] or [ERR]")]
    with_loglevels: bool,

    #[clap(
        long = "color",
        short = 'c',
        help = "make dates gray, [INFO] green and [ERR] red"
    )]
    with_colors: bool,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Output {
    StdOut,
    StdErr,
}

fn gen_random_string(mut rng: impl rand::Rng) -> String {
    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ01234567890 \t!\"#$%&'(),./\\;:@[]-^<>?_+*`{}=~|";
    random_string::generate(rng.gen_range(10..75), &charset)
}

fn main() {
    let args = Args::parse();
    let mut rng = rand::thread_rng();

    let iter_stdout = iter::repeat(Output::StdOut).take(args.stdout_lines);
    let iter_stderr = iter::repeat(Output::StdErr).take(args.stderr_lines);
    let iter_chain = iter_stdout.chain(iter_stderr);
    let shuffled: Vec<_> = {
        let mut v: Vec<_> = iter_chain.collect();
        v.shuffle(&mut rng);
        v
    };

    for output in shuffled {
        let date = if args.with_dates {
            let d = format!(
                "{}",
                chrono::Local::now().format("[%Y-%m-%d %H:%M:%S%.3f] ")
            );
            if args.with_colors {
                format!("{}", d.grey())
            } else {
                d
            }
        } else {
            "".to_owned()
        };

        let level = if args.with_loglevels {
            match (output, args.with_colors) {
                (Output::StdOut, true) => Cow::Owned(format!("{}", "[INFO] ".green())),
                (Output::StdOut, false) => Cow::Borrowed("[INFO] "),
                (Output::StdErr, true) => Cow::Owned(format!("{}", "[ERR] ".red())),
                (Output::StdErr, false) => Cow::Borrowed("[ERR] "),
            }
        } else {
            Cow::Borrowed("")
        };

        let body = gen_random_string(&mut rng);

        let s = date + level.borrow() + &body;

        match output {
            Output::StdOut => println!("{}", s),
            Output::StdErr => eprintln!("{}", s),
        }
        std::thread::sleep(std::time::Duration::from_millis(args.wait_ms));
    }

    std::process::exit(args.exit_code);
}
