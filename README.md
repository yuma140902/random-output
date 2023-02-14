# random-output

[![](https://badgen.net/crates/v/random-output?color=blue)](https://crates.io/crates/random-output)
[![](https://docs.rs/random-output/badge.svg)](https://docs.rs/random-output/)

Outputs random lines to stdout and stderr. Useful for tests for shell scripts and automations.

```
USAGE:
    random-output.exe [OPTIONS]

OPTIONS:
    -c, --color                      Make dates gray, [INFO] green and [ERR] red
    -d, --date                       Show dates in [%yyyy-%mm-%dd %HH:%MM:%SS.%3f] style
    -e <STDERR_LINES>                Number of stderr lines [default: 10]
        --exit <EXIT_CODE>           Exit code of the process [default: 0]
    -h, --help                       Print help information
    -l, --level                      Show [INFO] for stdout and [ERR] for stderr
    -n <STDOUT_LINES>                Number of stdout lines [default: 10]
        --prefix <PREFIX>            Prefix to add each line [default: ]
        --prefix-err <PREFIX_ERR>    Prefix to add each stderr line. Defaults to `--prefix`
        --suffix <SUFFIX>            Suffix to add each line [default: ]
        --suffix-err <SUFFIX_ERR>    Suffix to add each stderr line. Defaults to `--suffix`
    -V, --version                    Print version information
    -w, --working-dir                Show working dir in the first line
        --wait <WAIT_MS>             Specify how long to wait between outputs in <ms> [default: 0]
```
