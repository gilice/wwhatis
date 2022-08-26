use crate::utils::{convert, open::open_link};
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
    // throw away the first value since it's the executable name
    plain_args_it.next();

    if let Some(farg) = plain_args_it.next() {
        let mut args = vec![farg];
        args.extend(plain_args_it);

        let parsed = utils::cli::parse(&args, &out);
        let is_multiple: bool = parsed.topics.len() > 1;
        let mut queue: [String; 3] = Default::default();

        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, Body>(https);
        let mut spinner = Spinner::new(Spinners::Dots, "Loading...".to_string());

        let bodies = futures::future::join_all(parsed.topics.iter().map(|topic| {
            let client = &client;

            async move {
                let raw_uri = convert::topic_to_url(topic, parsed.lang);

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
                        &parsed.lang, location
                    );
                    resp = client.get(new_uri.parse::<Uri>().unwrap()).await.unwrap();
                }

                let bytes = body::to_bytes(resp.into_body()).await.unwrap();

                return String::from_utf8(bytes.to_vec())
                    .expect(format!("Response for {} was not valid utf-8", topic).as_str());
            }
        }))
        .await;
        let mut urls = Vec::new();
        for i in 0..bodies.len() {
            let parsed_response = json::parse(&bodies[i].as_str()).expect(
                format!("Could not parse response for {} as JSON", parsed.topics[i]).as_str(),
            );

            if is_multiple {
                let title = String::from(
                    parsed_response["titles"]["normalized"]
                        .as_str()
                        .unwrap_or("No title"),
                );
                let prefix = convert::add_prefix(parsed.formatless, title);

                queue[0] += prefix.as_str();
                queue[1] += prefix.as_str();
                queue[2] += prefix.as_str();
            }

            // add description to queue
            queue[0] += format!(
                "{}\n",
                convert::trim_and_shorten(
                    parsed_response["description"]
                        .as_str()
                        .unwrap_or("No description"),
                )
            )
            .as_str();

            // add extract to queue
            queue[1] += format!(
                "{}\n",
                convert::trim_and_shorten(
                    parsed_response["extract"].as_str().unwrap_or("No extract")
                )
            )
            .as_str();

            let current_url = convert::get_page_url(parsed.mobile, &parsed_response);

            queue[2] += format!("{}\n", current_url).as_str();
            urls.push(current_url);
        }

        spinner.stop();
        // clear the spinner line & print attribution
        print!("\x1b[2K\r\n{}\n---\n", utils::cli::ATTRIBUTION);

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
        utils::cli::help(&out);
        return;
    }
}
