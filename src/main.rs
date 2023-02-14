use clap::Parser;
use crossterm::style::Stylize;
use rand::seq::SliceRandom;
use random_output::Args;
use std::{
    borrow::{Borrow, Cow},
    iter,
};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Output {
    StdOut,
    StdErr,
}

fn gen_random_string(mut rng: impl rand::Rng) -> String {
    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ01234567890 \t!\"#$%&'(),./\\;:@[]-^<>?_+*`{}=~|";
    random_string::generate(rng.gen_range(10..75), charset)
}

#[derive(Debug)]
struct Modifier<'a> {
    dates: bool,
    loglevels: bool,
    colors: bool,
    prefix: &'a str,
    suffix: &'a str,
    prefix_err: Option<&'a str>,
    suffix_err: Option<&'a str>,
}

impl<'a> From<&'a Args> for Modifier<'a> {
    fn from(args: &'a Args) -> Self {
        Modifier {
            dates: args.with_dates,
            loglevels: args.with_dates,
            colors: args.with_colors,
            prefix: &args.prefix,
            suffix: &args.suffix,
            prefix_err: args.prefix_err.as_deref(),
            suffix_err: args.suffix_err.as_deref(),
        }
    }
}

fn add_modifier(line: &str, output: Output, modifier: &Modifier) -> String {
    let mut s = "".to_owned();

    let prefix_err = if let Some(prefix_err) = modifier.prefix_err {
        prefix_err
    } else {
        modifier.prefix
    };

    let prefix = match output {
        Output::StdOut => modifier.prefix,
        Output::StdErr => prefix_err,
    };

    let suffix_err = if let Some(suffix_err) = modifier.suffix_err {
        suffix_err
    } else {
        modifier.suffix
    };

    let suffix = match output {
        Output::StdOut => modifier.suffix,
        Output::StdErr => suffix_err,
    };

    if modifier.dates {
        let d = format!(
            "{}",
            chrono::Local::now().format("[%Y-%m-%d %H:%M:%S%.3f] ")
        );
        let d = if modifier.colors {
            format!("{}", d.grey())
        } else {
            d
        };
        s += &d;
    }

    s = prefix.to_string() + &s;

    if modifier.loglevels {
        let l = match (output, modifier.colors) {
            (Output::StdOut, true) => Cow::Owned(format!("{}", "[INFO] ".green())),
            (Output::StdOut, false) => Cow::Borrowed("[INFO] "),
            (Output::StdErr, true) => Cow::Owned(format!("{}", "[ERR] ".red())),
            (Output::StdErr, false) => Cow::Borrowed("[ERR] "),
        };
        s += l.borrow();
    }

    s += line;
    s += suffix;

    s
}

fn main() {
    let args = Args::parse();
    let mut rng = rand::thread_rng();

    if args.with_working_dir {
        let working_dir = std::env::current_dir()
            .map(|p| p.display().to_string())
            .unwrap_or("(error)".to_string());
        let msg = if args.with_colors {
            format!("Working directory: {}", working_dir.on_dark_magenta())
        } else {
            format!("Working directory: {}", working_dir)
        };
        println!(
            "{}",
            add_modifier(&msg, Output::StdOut, &Modifier::from(&args))
        );
    }

    let iter_stdout = iter::repeat(Output::StdOut).take(args.stdout_lines);
    let iter_stderr = iter::repeat(Output::StdErr).take(args.stderr_lines);
    let iter_chain = iter_stdout.chain(iter_stderr);
    let shuffled: Vec<_> = {
        let mut v: Vec<_> = iter_chain.collect();
        v.shuffle(&mut rng);
        v
    };

    for output in shuffled {
        let random = gen_random_string(&mut rng);
        let line = add_modifier(&random, output, &Modifier::from(&args));
        match output {
            Output::StdOut => println!("{}", line),
            Output::StdErr => eprintln!("{}", line),
        }
        std::thread::sleep(std::time::Duration::from_millis(args.wait_ms));
    }

    std::process::exit(args.exit_code);
}
