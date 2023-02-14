use clap::Parser;
use crossterm::style::Stylize;
use rand::seq::SliceRandom;
use std::iter;

use random_output::args::Args;
use random_output::modifier;
use random_output::modifier::Modifier;
use random_output::modifier::Output;

fn gen_random_string(mut rng: impl rand::Rng) -> String {
    let charset = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ01234567890 \t!\"#$%&'(),./\\;:@[]-^<>?_+*`{}=~|";
    random_string::generate(rng.gen_range(10..75), charset)
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
            format!("Working directory: {working_dir}")
        };
        println!(
            "{}",
            modifier::add_modifier_to_line(&msg, Output::StdOut, &Modifier::from(&args))
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
        let line = modifier::add_modifier_to_line(&random, output, &Modifier::from(&args));
        match output {
            Output::StdOut => println!("{line}"),
            Output::StdErr => eprintln!("{line}"),
        }
        std::thread::sleep(std::time::Duration::from_millis(args.wait_ms));
    }

    std::process::exit(args.exit_code);
}
