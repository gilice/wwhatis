use std::ffi::OsString;

use crossterm::style::Stylize;
use json::JsonValue;

pub fn parse_utf8(os_str: &OsString) -> &str {
    os_str
        .to_str()
        .unwrap_or_else(|| panic!("Could not parse argument {:?} as UTF8", &os_str))
}

pub fn trim_and_shorten(str: &str) -> String {
    let mut nw = str.replace('\n', " ");
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

pub fn bodies_to_queue(
    bodies: &Vec<String>,
    topics: &Vec<&str>,
    formatless: &bool,
    mobile: &bool,
) -> ([String; 3], Vec<String>) {
    let is_multiple = topics.len() > 1;
    let mut queue: [String; 3] = <[String; 3]>::default();
    let mut urls = vec![];
    for i in 0..bodies.len() {
        let parsed_response = json::parse(&bodies[i].as_str())
            .unwrap_or_else(|_| panic!("Could not parse response for {} as JSON", topics[i]));

        if is_multiple {
            let title = String::from(
                parsed_response["titles"]["normalized"]
                    .as_str()
                    .unwrap_or("No title"),
            );
            let prefix = add_prefix(*formatless, title);

            queue[0] += prefix.as_str();
            queue[1] += prefix.as_str();
            queue[2] += prefix.as_str();
        }

        // add description to queue
        queue[0] += format!(
            "{}\n",
            trim_and_shorten(
                parsed_response["description"]
                    .as_str()
                    .unwrap_or("No description"),
            )
        )
        .as_str();

        // add extract to queue
        queue[1] += format!(
            "{}\n",
            trim_and_shorten(parsed_response["extract"].as_str().unwrap_or("No extract"))
        )
        .as_str();

        let current_url = get_page_url(*mobile, &parsed_response);

        queue[2] += format!("{}\n", current_url).as_str();
        urls.push(current_url);
    }

    return (queue, urls);
}
