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
