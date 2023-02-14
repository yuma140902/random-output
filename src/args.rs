use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about = "Output random lines to stdout and stderr")]
pub struct Args {
    #[clap(short = 'n', help = "Number of stdout lines", default_value_t = 10)]
    pub stdout_lines: usize,

    #[clap(short = 'e', help = "Number of stderr lines", default_value_t = 10)]
    pub stderr_lines: usize,

    #[clap(
        long = "wait",
        help = "Specify how long to wait between outputs in <ms>",
        default_value_t = 0
    )]
    pub wait_ms: u64,

    #[clap(long = "prefix", help = "Prefix to add each line", default_value = "")]
    pub prefix: String,

    #[clap(long = "suffix", help = "Suffix to add each line", default_value = "")]
    pub suffix: String,

    #[clap(
        long = "prefix-err",
        help = "Prefix to add each stderr line. Defaults to `--prefix`"
    )]
    pub prefix_err: Option<String>,

    #[clap(
        long = "suffix-err",
        help = "Suffix to add each stderr line. Defaults to `--suffix`"
    )]
    pub suffix_err: Option<String>,

    #[clap(long = "exit", help = "Exit code of the process", default_value_t = 0)]
    pub exit_code: i32,

    #[clap(
        long = "date",
        short = 'd',
        help = "Show dates in [%yyyy-%mm-%dd %HH:%MM:%SS.%3f] style"
    )]
    pub with_dates: bool,

    #[clap(
        long = "level",
        short = 'l',
        help = "Show [INFO] for stdout and [ERR] for stderr"
    )]
    pub with_loglevels: bool,

    #[clap(
        long = "color",
        short = 'c',
        help = "Make dates gray, [INFO] green and [ERR] red"
    )]
    pub with_colors: bool,

    #[clap(
        long = "working-dir",
        short = 'w',
        help = "Show working dir in the first line"
    )]
    pub with_working_dir: bool,
}
