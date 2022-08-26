use std::process::Command;

pub fn open_link(url: &String) {
    Command::new("xdg-open")
        .arg(url)
        .output()
        .expect(format!("Could not display url {}", &url).as_str());
}
