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
        // let mut urls = Vec::new();
        // let mut queue: [String; 3] = Default::default();

        let (mut queue, urls) =
            convert::bodies_to_queue(&bodies, &parsed.topics, &parsed.formatless, &parsed.mobile);

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
