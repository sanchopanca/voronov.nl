use rand::Rng;
use warp::filters;
use warp::http::header::{CACHE_CONTROL, CONTENT_TYPE, REFRESH, SET_COOKIE};
use warp::Filter;

const REFRESH_TARGET: &str = "2;https://aleksei.nl";

const IP: [u8; 4] = [0, 0, 0, 0];
const PORT: u16 = 3030;

const COOKIE_NAME: &str = "five";

const CATS: [&'static str; 5] = [
    "lechat-1.png",
    "lechat-2.png",
    "lechat-3.png",
    "lechat-4.png",
    "lechat-5.png",
];

#[tokio::main]
async fn main() {
    let hello =
        warp::path::end()
            .and(filters::cookie::optional(COOKIE_NAME))
            .map(|maybe_cat: Option<usize>| {
                let mut cat_number = maybe_cat.unwrap_or(0);

                if cat_number >= CATS.len() {
                    cat_number = 0;
                }

                let html = format!(r#"
<title>Aleksei</title>

<style>
html, body {{
    padding: 0;
    margin: 0;
    width: 100%;
    height: 100%;
}}

body {{
    background: #c8b9a4;
    color: #2d2924;
    font-family: 'Cedarville Cursive', cursive;
    font-size: 7vh;
    overflow: hidden;
}}

p {{
    margin: 0;
    padding: 0;
    margin-top: 7vh;
    line-height: 7vh;
}}

div {{
    position: absolute;
    left: 50%;
    top: 50%;
    transform: translate(-50%, -50%);
    text-align: center;
}}

img {{
    height: 50vh;
}}
</style>

<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=Cedarville+Cursive&display=swap" rel="stylesheet">

<div>
    <img src="/resources/{}" alt="le chat" title="le chat">
    <p>
        Ceci n'est pas un Aleksei
    </p>
</div>
"#, CATS[cat_number]);

            cat_number += 1;

            warp::http::response::Builder::new()
                .header(CONTENT_TYPE, "text/html; charset=utf-8")
                .header(
                    SET_COOKIE,
                    format!("{}={}", COOKIE_NAME, cat_number),
                )
                .header(CACHE_CONTROL, "no-store")
                .header(REFRESH, REFRESH_TARGET)
                .status(200)
                .body(html)
                .unwrap()
        },
    );

    let resources = warp::path("resources").and(warp::fs::dir("resources"));

    warp::serve(resources.or(hello)).run((IP, PORT)).await;
}
