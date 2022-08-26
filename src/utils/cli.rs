use std::{
    ffi::OsString,
    io::{Stdout, Write},
    process,
};

use crate::utils::convert::parse_utf8;

pub static HELP: &[u8] = r#"wwhatis-rs 0.1.0
    Quickly displays summaries of given topics. This program displays text from Wikipedia, the Free
    Encyclopedia. Check it out at https://wikipedia.org
    
    USAGE:
        wwhatis [OPTIONS] [TOPICS]...
    
    ARGS:
        <TOPICS>...    
    
    OPTIONS:
        -a, --about          Print the about page and info about used libraries
        -f, --formatless     Enable if running in a terminal that doesn't support ANSI escape codes
        -h, --help           Print help information
        -l, --lang <LANG>    The language prefix of the wiki's url that you want to use, like "en" or
                             "de" [default: en]
        -m, --mobile         Return URL in mobile (m.wikipedia.org) version
        -o, --open           Open the article URLs on the last step in your default web browser with
                             xdg-open (only works on *nix)
        -V, --version        Print version information
    "#.as_bytes();

pub static ATTRIBUTION: &str = r#"From Wikipedia, the Free Encyclopedia. License: https://creativecommons.org/licenses/by-sa/3.0/"#;

pub fn help(mut out: &Stdout) {
    out.write(HELP).unwrap();
    return;
}

pub fn parse<'a>(args: &'a Vec<OsString>, out: &'a Stdout) -> Arguments<'a> {
    let mut ret_args = Arguments {
        formatless: false,
        mobile: false,
        open: false,
        lang: "en",
        topics: vec![],
    };

    for arg in args {
        let parsed = parse_utf8(&arg);
        match parsed {
            "--help" | "-h" => {
                help(&out);
                process::exit(1);
            }

            "--about" | "-a" => {
                let tpty = include_str!("../../thirdparty/THIRDPARTY");
                println!("{}", tpty);
                process::exit(1);
            }

            "--formatless" | "-f" => ret_args.formatless = true,
            "--mobile" | "-m" => ret_args.mobile = true,
            "--open" | "-o" => ret_args.open = true,
            "--lang" | "-l" => ret_args.lang = parsed,
            _ => ret_args.topics.push(parsed),
        }
    }

    if ret_args.topics.is_empty() {
        eprintln!("No topics provided. Execute with -h to display usage.");
        process::exit(1);
    }

    return ret_args;
}

pub struct Arguments<'a> {
    pub topics: Vec<&'a str>,

    /// The language prefix of the wiki's url that you want to use, like "en" or "de"
    pub lang: &'a str,

    /// Return URL in mobile (m.wikipedia.org) version
    pub mobile: bool,

    /// Enable if running in a terminal that doesn't support ANSI escape codes
    pub formatless: bool,

    /// Open the article URLs on the last step in your default web browser with xdg-open (only works on *nix)
    pub open: bool,
}
