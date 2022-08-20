use clap::Parser;
use crossterm::style::Stylize;
use hyper::{body, header, Body, Client, Response, Uri};
use hyper_tls::HttpsConnector;
use spinners::{Spinner, Spinners};
use std::process::Command;
mod model;

extern crate dont_disappear;
extern crate hyper;
extern crate tokio;

#[tokio::main]
async fn main() {
    let args: Input = Input::parse();
    if args.about {
        let tpty = include_str!("../thirdparty/THIRDPARTY");
        println!("{}", tpty);
        return;
    }
    println!("{}", "hello, world");
    if args.topics.is_empty() {
        eprintln!("No topics provided. Execute with -h to display usage.");
        return;
    }

    let is_multiple: bool = args.topics.len() > 1;
    let mut queue: [String; 3] = Default::default();
    let https = HttpsConnector::new();

    let client = Client::builder().build::<_, Body>(https);

    let mut spinner = Spinner::new(Spinners::Dots, "Loading...".into());
    let bodies = futures::future::join_all(args.topics.into_iter().map(|url| {
        let client = &client;
        let lang_clone = &args.lang;
        let url_clone = url;
        async move {
            let url_encoded = urlencoding::encode(url_clone.as_str());

            let raw_uri = format!(
                "https://{}.wikipedia.org/api/rest_v1/page/summary/{}",
                lang_clone, url_encoded
            );

            let mut resp: Response<Body> =
                client.get(raw_uri.parse::<Uri>().unwrap()).await.unwrap();

            let status = resp.status();

            // follow redirects.
            if status == 301 || status == 302 {
                let location = resp
                    .headers()
                    .get(header::LOCATION)
                    .unwrap()
                    .to_str()
                    .unwrap();
                let new_uri = format!(
                    "https://{}.wikipedia.org/api/rest_v1/page/summary/{}",
                    lang_clone, location
                );
                resp = client.get(new_uri.parse::<Uri>().unwrap()).await.unwrap();
            }

            let bytes = body::to_bytes(resp.into_body()).await.unwrap();
            let stri = String::from_utf8(bytes.to_vec()).expect("response was not valid utf-8");
            return stri;
        }
    }))
    .await;
    let mut urls = Vec::<String>::new();
    for body in bodies {
        // parse response

        let parsed_res: model::summary_response::SummaryResponse =
            serde_json::from_str(body.as_str()).unwrap();

        if is_multiple {
            let title = parsed_res.titles.display;

            let prefix = format!(
                "{}: ",
                if args.formatless {
                    title
                } else {
                    title.bold().green().to_string()
                }
            );

            queue[0] += prefix.as_str();
            queue[1] += prefix.as_str();
            queue[2] += prefix.as_str();
        }
        let mut desc = parsed_res.description.replace("\n", " ");
        desc.retain(|c| (!c.is_whitespace() || c == ' '));

        queue[0] += format!("{}\n", desc).as_str();

        let mut ext = parsed_res.extract.replace("\n", " ");
        ext.retain(|c| (!c.is_whitespace() || c == ' '));

        queue[1] += format!("{}\n", ext).as_str();

        let current_url = if args.mobile {
            parsed_res.content_urls.mobile.page
        } else {
            parsed_res.content_urls.desktop.page
        };
        queue[2] += format!("{}\n", current_url).as_str();

        urls.push(current_url);
    }
    spinner.stop();
    // clear the spinner line.
    print!("\x1b[2K\r\nFrom Wikipedia, the Free Encyclopedia. License: https://creativecommons.org/licenses/by-sa/3.0/\n---\n");

    for i in 0..2 {
        let sstr = &mut queue[i];
        dont_disappear::any_key_to_continue::custom_msg(&sstr);
        println!();
    }

    print!("{}", queue[2]);
    if args.open {
        for url in urls {
            let urlstr = url.as_str();
            Command::new("xdg-open")
                .arg(urlstr)
                .output()
                .expect(format!("Could not display url {}", urlstr).as_str());
        }
    }
}

#[derive(Parser, Default, Debug)]
/// Quickly displays summaries of given topics. This program displays text from Wikipedia, the Free Encyclopedia. Check it out at https://wikipedia.org.

#[clap(version, about)]
struct Input {
    #[clap(forbid_empty_values = true)]
    topics: Vec<String>,

    /// The language prefix of the wiki's url that you want to use, like "en" or "de"
    #[clap(short, long, default_value = "en")]
    lang: String,

    /// Return URL in mobile (m.wikipedia.org) version
    #[clap(short, long, takes_value = false)]
    mobile: bool,

    /// Enable if running in a terminal that doesn't support ANSI escape codes
    #[clap(short, long, takes_value = false)]
    formatless: bool,

    /// Open the article URLs on the last step in your default web browser with xdg-open (only works on *nix)
    #[clap(short, long, takes_value = false)]
    open: bool,

    /// Print the about page and info about used libraries
    #[clap(short, long, takes_value = false)]
    about: bool,
}
