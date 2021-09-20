use rand::Rng;
use warp::filters;
use warp::http::header::{CACHE_CONTROL, CONTENT_TYPE, REFRESH, SET_COOKIE};
use warp::Filter;

const REFRESH_TARGET: &str = "1;https://aleksei.nl";

const IP: [u8; 4] = [0, 0, 0, 0];
const PORT: u16 = 3030;

const COOKIE_NAME: &str = "four";
const MAX_COUNTERS: usize = 10;

fn deserialize_counters(string: String) -> Vec<usize> {
    string
        .split(",")
        .filter_map(|counter_str| counter_str.parse().ok())
        .collect()
}

fn serialize_counters(counters: &Vec<usize>) -> String {
    let mut string = String::new();

    let mut counters_iter = counters.into_iter().map(|c| c.to_string());

    match counters_iter.next() {
        Some(counter_str) => string.push_str(&counter_str),
        None => return string,
    };

    for counter_str in counters_iter {
        string.push(',');
        string.push_str(&counter_str);
    }

    string
}

#[tokio::main]
async fn main() {
    let hello = warp::any().and(filters::cookie::optional(COOKIE_NAME)).map(
        |counters_string: Option<String>| {
            let mut rng = rand::thread_rng();

            let mut counters = counters_string
                .map(deserialize_counters)
                .unwrap_or(Vec::new());

            counters.push(rng.gen_range(2..25));

            let mut html = r#"
<title>Aleksei</title>

<style>
html, body {
    padding: 0;
    margin: 0;
    width: 100%;
    height: 100%;
}

body {
    background: black;
    color: white;
    font-family: serif;
    white-space: nowrap;
    overflow: hidden;
    font-size: 10vh;
    line-height: 10vh;
}

p {
    margin: 0;
    padding: 0;
}
</style>

"#
            .to_string();

            for number in &counters {
                html.push_str(&format!("<p>{}</p>", "A".repeat(*number)));
            }

            html.push_str("<p>Aleksei</p>");

            if counters.len() >= MAX_COUNTERS {
                counters = Vec::new();
            }

            warp::http::response::Builder::new()
                .header(CONTENT_TYPE, "text/html; charset=utf-8")
                .header(
                    SET_COOKIE,
                    format!("{}={}", COOKIE_NAME, serialize_counters(&counters)),
                )
                .header(CACHE_CONTROL, "no-store")
                .header(REFRESH, REFRESH_TARGET)
                .status(200)
                .body(html)
                .unwrap()
        },
    );

    warp::serve(hello).run((IP, PORT)).await;
}
