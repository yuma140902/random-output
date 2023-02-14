use crate::args::Args;
use crate::time;

use crossterm::style::Stylize;
use std::borrow::{Borrow, Cow};

#[derive(Debug)]
pub struct Modifier<'a> {
    pub dates: bool,
    pub loglevels: bool,
    pub colors: bool,
    pub prefix: &'a str,
    pub suffix: &'a str,
    pub prefix_err: Option<&'a str>,
    pub suffix_err: Option<&'a str>,
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

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Output {
    StdOut,
    StdErr,
}

#[must_use] pub fn add_modifier_to_line(line: &str, output: Output, modifier: &Modifier) -> String {
    let mut s = String::new();

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
        let d = format!("{}", time::now().format("[%Y-%m-%d %H:%M:%S%.3f] "));
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

#[cfg(test)]
mod test {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn null_modifier_does_nothing() {
        let modifier = Modifier {
            dates: false,
            loglevels: false,
            colors: false,
            prefix: "",
            suffix: "",
            prefix_err: None,
            suffix_err: None,
        };

        for output in [Output::StdOut, Output::StdErr] {
            assert_eq!(
                add_modifier_to_line("hogehoge", output, &modifier),
                "hogehoge"
            );
        }
    }

    #[test]
    fn prefix_and_suffix() {
        let modifier = Modifier {
            dates: false,
            loglevels: false,
            colors: false,
            prefix: "prefix",
            suffix: "suffix",
            prefix_err: Some("prefix-err"),
            suffix_err: Some("suffix-err"),
        };

        assert_eq!(
            add_modifier_to_line("hoge", Output::StdOut, &modifier),
            "prefixhogesuffix"
        );
        assert_eq!(
            add_modifier_to_line("hoge", Output::StdErr, &modifier),
            "prefix-errhogesuffix-err"
        );
    }

    #[test]
    fn prefix_err_and_suffix_err_defaults_to_prefix_and_suffix() {
        let modifier = Modifier {
            dates: false,
            loglevels: false,
            colors: false,
            prefix: "prefix",
            suffix: "suffix",
            prefix_err: None,
            suffix_err: None,
        };

        assert_eq!(
            add_modifier_to_line("hoge", Output::StdErr, &modifier),
            "prefixhogesuffix"
        );
    }

    #[test]
    fn loglevels() {
        let modifier = Modifier {
            dates: false,
            loglevels: true,
            colors: false,
            prefix: "",
            suffix: "",
            prefix_err: None,
            suffix_err: None,
        };

        assert_eq!(
            add_modifier_to_line("hoge", Output::StdOut, &modifier),
            "[INFO] hoge"
        );
        assert_eq!(
            add_modifier_to_line("hoge", Output::StdErr, &modifier),
            "[ERR] hoge"
        );
    }

    #[test]
    fn prefix_dates_loglevels_come_in_order() {
        let dt: chrono::NaiveDateTime = "2022-09-02T10:11:12".parse().unwrap();
        let ldt = chrono::Local.from_local_datetime(&dt).unwrap();
        time::mock_time::set_mock_time(ldt);

        let modifier = Modifier {
            dates: true,
            loglevels: true,
            colors: false,
            prefix: "prefix",
            suffix: "",
            prefix_err: Some("prefix"),
            suffix_err: None,
        };

        assert_eq!(
            add_modifier_to_line("hoge", Output::StdOut, &modifier),
            "prefix[2022-09-02 10:11:12.000] [INFO] hoge"
        );
        assert_eq!(
            add_modifier_to_line("hoge", Output::StdErr, &modifier),
            "prefix[2022-09-02 10:11:12.000] [ERR] hoge"
        );
    }
}
