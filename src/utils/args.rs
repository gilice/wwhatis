use std::{
    ffi::OsString,
    io::{Stdout, Write},
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

pub fn help(mut out: &Stdout) {
    out.write(HELP).unwrap();
    return;
}

pub fn parse(args: &Vec<OsString>, out: &Stdout) -> &Arguments {
    let mut ret_args = Arguments {
        formatless: false,
        mobile: false,
        open: false,
        lang: "en",
        topics: vec![],
    };

    for arg in args {
        dbg!(&arg);
        let parsed = parse_utf8(&arg);
        match parsed {
            "--help" | "-h" => {
                help(&out);
                return;
            }

            "--about" | "-a" => {
                let tpty = include_str!("../../thirdparty/THIRDPARTY");
                println!("{}", tpty);
                return;
            }

            "--formatless" | "-f" => ret_args.formatless = true,
            "--mobile" | "-m" => ret_args.mobile = true,
            "--open" | "-o" => ret_args.open = true,
            "--lang" | "-l" => ret_args.lang = parsed,
            _ => ret_args.topics.push(parsed),
        }
    }
    return &ret_args;
}

struct Arguments<'a> {
    topics: Vec<&'a str>,

    /// The language prefix of the wiki's url that you want to use, like "en" or "de"
    lang: &'a str,

    /// Return URL in mobile (m.wikipedia.org) version
    mobile: bool,

    /// Enable if running in a terminal that doesn't support ANSI escape codes
    formatless: bool,

    /// Open the article URLs on the last step in your default web browser with xdg-open (only works on *nix)
    open: bool,
}
