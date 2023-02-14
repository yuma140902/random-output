use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about = "Output random lines to stdout and stderr")]
pub struct Args {
    #[clap(long = "n", short = 'n', default_value_t = 10)]
    pub stdout_lines: usize,

    #[clap(long = "e", short = 'e', default_value_t = 10)]
    pub stderr_lines: usize,

    #[clap(
        long = "wait",
        default_value_t = 0,
        help = "wait millisecond between outputs"
    )]
    pub wait_ms: u64,

    #[clap(long = "name", help = "name to show in output")]
    pub name: Option<String>,

    #[clap(long = "prefix", help = "prefix to add each line", default_value = "")]
    pub prefix: String,

    #[clap(long = "suffix", help = "suffix to add each line", default_value = "")]
    pub suffix: String,

    #[clap(long = "exit", default_value_t = 0)]
    pub exit_code: i32,

    #[clap(long = "date", short = 'd', help = "show dates")]
    pub with_dates: bool,

    #[clap(long = "level", short = 'l', help = "show [INFO] or [ERR]")]
    pub with_loglevels: bool,

    #[clap(
        long = "color",
        short = 'c',
        help = "make dates gray, [INFO] green and [ERR] red"
    )]
    pub with_colors: bool,

    #[clap(long = "working-dir", short = 'w', help = "output working dir")]
    pub with_working_dir: bool,
}
