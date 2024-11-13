use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about = "Output random lines to stdout and stderr")]
pub struct Args {
    #[arg(short = 'n', help = "Number of stdout lines", default_value_t = 10)]
    pub stdout_lines: usize,

    #[arg(short = 'e', help = "Number of stderr lines", default_value_t = 10)]
    pub stderr_lines: usize,

    #[arg(
        long = "wait",
        help = "Specify how long to wait between outputs in <ms>",
        default_value_t = 0
    )]
    pub wait_ms: u64,

    #[arg(long = "prefix", help = "Prefix to add each line", default_value = "")]
    pub prefix: String,

    #[arg(long = "suffix", help = "Suffix to add each line", default_value = "")]
    pub suffix: String,

    #[arg(
        long = "prefix-err",
        help = "Prefix to add each stderr line. Defaults to `--prefix`"
    )]
    pub prefix_err: Option<String>,

    #[arg(
        long = "suffix-err",
        help = "Suffix to add each stderr line. Defaults to `--suffix`"
    )]
    pub suffix_err: Option<String>,

    #[arg(long = "exit", help = "Exit code of the process", default_value_t = 0)]
    pub exit_code: i32,

    #[arg(
        long = "date",
        short = 'd',
        help = "Show dates in [%yyyy-%mm-%dd %HH:%MM:%SS.%3f] style"
    )]
    pub with_dates: bool,

    #[arg(
        long = "level",
        short = 'l',
        help = "Show [INFO] for stdout and [ERR] for stderr"
    )]
    pub with_loglevels: bool,

    #[arg(
        long = "color",
        short = 'c',
        help = "Make dates gray, [INFO] green and [ERR] red"
    )]
    pub with_colors: bool,

    #[arg(
        long = "working-dir",
        short = 'w',
        help = "Show working dir in the first line"
    )]
    pub with_working_dir: bool,
}
