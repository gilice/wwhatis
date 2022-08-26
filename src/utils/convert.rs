use std::ffi::OsString;

use crossterm::style::Stylize;
use json::JsonValue;

pub fn parse_utf8(os_str: &OsString) -> &str {
    os_str
        .to_str()
        .expect(format!("Could not parse argument {:?} as UTF8", &os_str).as_str())
}

pub fn trim_and_shorten(str: &str) -> String {
    let mut nw = str.replace("\n", " ");
    nw.retain(|c| (!c.is_whitespace() || c == ' '));
    return nw;
}

pub fn get_page_url(mobile: bool, parsed_response: &JsonValue) -> String {
    return if mobile {
        parsed_response["content_urls"]["mobile"]["page"]
            .as_str()
            .unwrap_or("No mobile url")
            .to_string()
    } else {
        parsed_response["content_urls"]["desktop"]["page"]
            .as_str()
            .unwrap_or("No desktop url")
            .to_string()
    };
}

pub fn topic_to_url(topic: &str, lang: &str) -> String {
    let url_encoded = urlencoding::encode(topic);

    return format!(
        "https://{}.wikipedia.org/api/rest_v1/page/summary/{}",
        lang, url_encoded
    );
}

pub fn add_prefix(formatless: bool, title: String) -> String {
    return format!(
        "{}: ",
        if formatless {
            title
        } else {
            title.bold().green().to_string()
        }
    );
}
