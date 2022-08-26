use std::io::{Stdout, Write};

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
