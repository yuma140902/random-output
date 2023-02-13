# random-output

[![](https://badgen.net/crates/v/random-output?color=blue)](https://crates.io/crates/random-output)
[![](https://docs.rs/random-output/badge.svg)](https://docs.rs/random-output/)

Output random lines to stdout and stderr. Useful for tests for shell scripts and automations.

```
USAGE:
    random-output.exe [OPTIONS]

OPTIONS:
    -c, --color               make dates gray, [INFO] green and [ERR] red
    -d, --date                show dates
    -e, --e <STDERR_LINES>    [default: 10]
        --exit <EXIT_CODE>    [default: 0]
    -h, --help                Print help information
    -l, --level               show [INFO] or [ERR]
    -n, --n <STDOUT_LINES>    [default: 10]
        --name <NAME>         name to show in output
    -V, --version             Print version information
    -w, --working-dir         output working dir
        --wait <WAIT_MS>      wait millisecond between outputs [default: 0]
```