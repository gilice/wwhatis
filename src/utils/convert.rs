use std::ffi::OsString;

pub fn parse_utf8(os_str: &OsString) -> &str {
    os_str
        .to_str()
        .expect(format!("Could not parse argument {:?} as UTF8", &os_str).as_str())
}
