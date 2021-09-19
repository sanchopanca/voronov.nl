use warp::filters;
use warp::http::header::{CACHE_CONTROL, CONTENT_TYPE, REFRESH, SET_COOKIE};
use warp::Filter;

const REFRESH_TARGET: &str = "2;https://aleksei.nl";

const IP: [u8; 4] = [0, 0, 0, 0];
const PORT: u16 = 3030;

const COOKIE_NAME: &str = "three";

#[tokio::main]
async fn main() {
    let hello = warp::any()
        .and(filters::cookie::optional(COOKIE_NAME))
        .map(|_: Option<String>| {
            let html = r#"
<title>Aleksei</title>

<style>
@import url(https://fonts.googleapis.com/css?family=Exo+2:200i);

html {
    --neon-text-color: #f40;
    --neon-border-color: #08f;
}

html, body {
    padding: 0;
    margin: 0;
    width: 100%;
    height: 100%;
}

body {
    background: black;
    color: white;
    font-family: 'Exo 2', sans-serif;
    white-space: nowrap;
    font-style: italic;
    overflow: hidden;
    font-size: 20vh;
    letter-spacing: -0.1vh;
}

span {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    animation: flicker 1s infinite alternate;
    padding: 4rem 6rem 5.5rem;
    border: 0.4rem solid #fff;
    border-radius: 2rem;
    text-transform: uppercase;
}

@keyframes flicker {
    0%, 19%, 21%, 23%, 25%, 54%, 56%, 100% {

        text-shadow:
            -0.2rem -0.2rem 1rem #fff,
            0.2rem 0.2rem 1rem #fff,
            0 0 2rem var(--neon-text-color),
            0 0 4rem var(--neon-text-color),
            0 0 6rem var(--neon-text-color),
            0 0 8rem var(--neon-text-color),
            0 0 10rem var(--neon-text-color);

        box-shadow:
            0 0 .5rem #fff,
            inset 0 0 .5rem #fff,
            0 0 2rem var(--neon-border-color),
            inset 0 0 2rem var(--neon-border-color),
            0 0 4rem var(--neon-border-color),
            inset 0 0 4rem var(--neon-border-color);
    }

    20%, 24%, 55% {
        text-shadow: none;
        box-shadow: none;
    }
}
</style>

<span>
Aleksei
</span>
"#
            .to_string();

            warp::http::response::Builder::new()
                .header(CONTENT_TYPE, "text/html; charset=utf-8")
                .header(SET_COOKIE, format!("{}={}", COOKIE_NAME, ""))
                .header(CACHE_CONTROL, "no-store")
                .header(REFRESH, REFRESH_TARGET)
                .status(200)
                .body(html)
                .unwrap()
        });

    warp::serve(hello).run((IP, PORT)).await;
}
