use rand::Rng;
use warp::filters;
use warp::http::header::{CACHE_CONTROL, CONTENT_TYPE, REFRESH, SET_COOKIE};
use warp::Filter;

const REFRESH_TARGET: &str = "1;https://aleksei.nl";

const IP: [u8; 4] = [0, 0, 0, 0];
const PORT: u16 = 3030;

const COOKIE_NAME: &str = "two";
const MIN_FONT_SIZE: f32 = 16.0;
const MAX_FONT_SIZE: f32 = 300.0;
const VORONOVS_FOR_A_STAR: usize = 60;
const MAX_VORONOVS: usize = 80;

struct Aleksei {
    x: f32,
    y: f32,
    font_size: f32,
}

fn deserialize_alekseis(string: String) -> Vec<Aleksei> {
    string
        .split("|")
        .filter_map(|aleksei_str| {
            let mut values_str = aleksei_str.split(',');

            let x = values_str.next().and_then(|x_str| x_str.parse().ok());
            let y = values_str.next().and_then(|y_str| y_str.parse().ok());
            let font_size = values_str.next().and_then(|fs_str| fs_str.parse().ok());

            let x = match x {
                None => return None,
                Some(x) => x,
            };

            let y = match y {
                None => return None,
                Some(y) => y,
            };

            let font_size = match font_size {
                None => return None,
                Some(font_size) => font_size,
            };

            if let Some(_) = values_str.next() {
                return None;
            }

            Some(Aleksei { x, y, font_size })
        })
        .collect()
}

fn serialize_alekseis(alekseis: &Vec<Aleksei>) -> String {
    let mut string = String::new();

    let mut alekseis_iter = alekseis
        .into_iter()
        .map(|a| format!("{},{},{}", a.x, a.y, a.font_size));

    match alekseis_iter.next() {
        Some(aleksei_str) => string.push_str(&aleksei_str),
        None => return string,
    };

    for aleksei_str in alekseis_iter {
        string.push('|');
        string.push_str(&aleksei_str);
    }

    string
}

#[tokio::main]
async fn main() {
    let hello = warp::any().and(filters::cookie::optional(COOKIE_NAME)).map(
        |serialized_alekseis: Option<String>| {
            let mut rng = rand::thread_rng();

            let mut alekseis = serialized_alekseis
                .map(deserialize_alekseis)
                .unwrap_or(Vec::new());

            alekseis.push(Aleksei {
                x: rng.gen(),
                y: rng.gen(),
                font_size: rng.gen_range(MIN_FONT_SIZE..MAX_FONT_SIZE),
            });

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
    font-family: sans-serif;
    font-weight: bold;
    white-space: nowrap;
    overflow: hidden;
}

span {
    position: absolute;
    transform: translate(-50%, -50%);
}

.star {
    left: 50%;
    top: 50%;
    font-size: 150px;
}
</style>
"#
            .to_string();

            for aleksei in &alekseis {
                html.push_str(&format!(
                    "<span style=\"left: {}%; top: {}%; font-size: {}px\">Aleksei</span>",
                    aleksei.x * 100.0,
                    aleksei.y * 100.0,
                    aleksei.font_size
                ));
            }

            if alekseis.len() >= VORONOVS_FOR_A_STAR {
                html.push_str("<span class=\"star\">⭐️</span>");
            }

            if alekseis.len() >= MAX_VORONOVS {
                alekseis = Vec::new();
            }

            warp::http::response::Builder::new()
                .header(CONTENT_TYPE, "text/html; charset=utf-8")
                .header(
                    SET_COOKIE,
                    format!("{}={}", COOKIE_NAME, serialize_alekseis(&alekseis)),
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
