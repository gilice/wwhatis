use std::process::Command;

pub fn open_link(url: &String) {
    Command::new("xdg-open")
        .arg(url)
        .output()
        .unwrap_or_else(|_| panic!("Could not display url {}", &url));
}
