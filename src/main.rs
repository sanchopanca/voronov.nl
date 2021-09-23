use warp::http::header::{CACHE_CONTROL, CONTENT_TYPE, REFRESH};
use warp::Filter;

const IP: [u8; 4] = [0, 0, 0, 0];
const PORT: u16 = 3030;

const FRAMES: &'static [Frame] = &[
    Frame::Dialogue("Hey.", 2),
    Frame::Dialogue("Sure.", 2),
    Frame::Dialogue("I work at the bank.", 2),
    Frame::Dialogue("Not very much, no.", 2),
    Frame::Dialogue("I’d like to be an artist", 2),
    Frame::Dialogue("I don’t know how to.", 2),
    Frame::Dialogue("Yes, very much.", 2),
    Frame::Art("good-art-1.png"),
    Frame::Art("good-art-2.jpg"),
    Frame::Dialogue("I’m afraid.", 2),
    Frame::Dialogue("Okay, I will do it.", 2),
    Frame::Dialogue("Oh wow! That was nice.", 2),
    Frame::Art("weirder-cat-1.jpg"),
    Frame::Art("weirder-cat-3.jpg"),
    Frame::Art("weirder-cat-5.jpg"),
    Frame::Art("weirder-cat-6.jpg"),
    Frame::End,
];

enum Frame {
    Dialogue(&'static str, u8),
    Art(&'static str),
    End,
}

#[tokio::main]
async fn main() {
    let init =
        warp::path::end()
            .map(|| {
                warp::http::response::Builder::new()
                    .header(CONTENT_TYPE, "text/html; charset=utf-8")
                    .header(CACHE_CONTROL, "no-store")
                    .header(REFRESH, "1;https://aleksei.nl/0")
                    .status(200)
                    .body("")
                    .unwrap()
            });

    let frame =
        warp::path::param::<usize>()
            .and(warp::path::end())
            .map(|n: usize| {
                let n = if n > FRAMES.len() { 0 } else { n };

                let (frame_html, delay) = match FRAMES[n] {
                    Frame::Dialogue(text, delay) => (format!("<div class=\"dialogue\">{}</div>", text), delay),
                    Frame::Art(filename) => (format!("<img src=\"/resources/{}\"/>", filename), 3),
                    Frame::End => ("<div class=\"fin\">fin</div>".to_owned(), 5),
                };

                let html =
                    format!(r#"
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
<link href="https://fonts.googleapis.com/css2?family=Yaldevi&display=swap" rel="stylesheet">

<title>Aleksei</title>

<style>
html, body {{
    padding: 0;
    margin: 0;
    width: 100%;
    height: 100%;
}}

body {{
    background: white;
    color: #222;
    overflow: hidden;
    text-align: center;
    font-size: 5vh;
    font-family: 'Yaldevi', serif;
    line-height: 5vh;
}}

.dialogue {{
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    max-width: 100%;
}}

img {{
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    height: 50vh;
}}

.fin {{
    position: absolute;
    font-size: 10vh;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
}}
</style>

{}
"#, frame_html);

                let refresh_target = format!("{};https://aleksei.nl/{}", delay, n + 1);

                warp::http::response::Builder::new()
                    .header(CONTENT_TYPE, "text/html; charset=utf-8")
                    .header(CACHE_CONTROL, "no-store")
                    .header(REFRESH, refresh_target)
                    .status(200)
                    .body(html)
                    .unwrap()
            });

    let resources = warp::path("resources").and(warp::fs::dir("resources"));

    warp::serve(resources.or(init).or(frame)).run((IP, PORT)).await;
}
