use crate::utils::open::open_link;
use crossterm::style::Stylize;
use hyper::{body, header, Body, Client, Response, Uri};
use hyper_tls::HttpsConnector;
use spinners::{Spinner, Spinners};
use std::{env, io::stdout};
extern crate dont_disappear;
extern crate hyper;
extern crate tokio;
mod utils;
#[tokio::main]
async fn main() {
    let out = stdout();

    let mut plain_args_it = env::args_os();
    plain_args_it.next();

    if let Some(farg) = plain_args_it.next() {
        let (mut args, mut topics) = (vec![farg], vec![]);
        args.extend(plain_args_it);
        let parsed = utils::args::parse(&args, &out);

        //let args: Input = Input::parse();

        if topics.is_empty() {
            eprintln!("No topics provided. Execute with -h to display usage.");
            return;
        }

        let is_multiple: bool = topics.len() > 1;
        let mut queue: [String; 3] = Default::default();
        let https = HttpsConnector::new();

        let client = Client::builder().build::<_, Body>(https);

        let mut spinner = Spinner::new(Spinners::Dots, "Loading...".into());
        let bodies = futures::future::join_all(topics.into_iter().map(|url| {
            let client = &client;
            let lang_clone = parsed.lang;
            let url_clone = parsed.url;

            async move {
                let url_encoded = urlencoding::encode(url_clone);

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
        let mut urls = Vec::new();
        for body in bodies {
            let parsed_response =
                json::parse(body.as_str()).expect("Could not parse response as JSON");

            if is_multiple {
                let title = String::from(
                    parsed_response["titles"]["normalized"]
                        .as_str()
                        .unwrap_or("No title"),
                );
                let prefix = format!(
                    "{}: ",
                    if parsed.formatless {
                        title
                    } else {
                        title.bold().green().to_string()
                    }
                );

                queue[0] += prefix.as_str();
                queue[1] += prefix.as_str();
                queue[2] += prefix.as_str();
            }

            let mut desc = parsed_response["description"]
                .as_str()
                .unwrap_or("No description")
                .replace("\n", " ");
            desc.retain(|c| (!c.is_whitespace() || c == ' '));

            queue[0] += format!("{}\n", desc).as_str();

            let mut ext = parsed_response["extract"]
                .as_str()
                .unwrap_or("No extract")
                .replace("\n", " ");
            ext.retain(|c| (!c.is_whitespace() || c == ' '));

            queue[1] += format!("{}\n", ext).as_str();
            let current_url = if parsed.mobile {
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

        if parsed.open {
            for url in &urls {
                open_link(url);
            }
        }
    } else {
        // No options provided, print help and exit
        utils::args::help(&out);
        return;
    }
}
