use axum::{http::StatusCode, response::Html, routing::get, Router};

#[tokio::main]
pub async fn main() {
    // router -------------------------------------------------------
    let app: Router = axum::Router::new()
        // .fallback( fallback.into_service() )
        .fallback(fallback) // different from tutorial; whew
        .route("/", get(hello))
        .route("/demo_from_string.html", get(get_demo_html_from_string))
        .route(
            "/demo_from_sibling_file.html",
            get(get_demo_html_from_sibling_file),
        )
        .route(
            "/demo_from_html_sub_dir.html",
            get(get_demo_html_from_sub_dir),
        )
        .route("/demo_status_code", get(get_demo_status_code))
        .route("/demo-uri", get(demo_uri))
        .route("/demo.png", get(get_demo_png));

    // run app as hyper server on http://localhost:3000 -------------
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
} // end main()

// route handlers ---------------------------------------------------

pub async fn fallback(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    (
        axum::http::StatusCode::NOT_FOUND,
        format!("No route {}", uri),
    )
}

pub async fn hello() -> String {
    "Hello, World!".into()
}

pub async fn get_demo_html_from_string() -> Html<&'static str> {
    "<h1>Hello from html-string</h1>".into()
}

pub async fn get_demo_html_from_sibling_file() -> Html<&'static str> {
    include_str!("./hello_from_sibling_file.html").into()
}

pub async fn get_demo_html_from_sub_dir() -> Html<&'static str> {
    include_str!("./html/hello_from_sub_dir.html").into()
}

pub async fn get_demo_status_code() -> (StatusCode, String) {
    (axum::http::StatusCode::OK, "200/ OK".to_string())
}

pub async fn demo_uri(uri: axum::http::Uri) -> String {
    format!("the uri is, ``{:?}``", uri)
}

pub async fn get_demo_png() -> impl axum::response::IntoResponse {
    let png = concat!(
        "iVBORw0KGgoAAAANSUhEUgAAAAEAAAAB",
        "CAYAAAAfFcSJAAAADUlEQVR42mPk+89Q",
        "DwADvgGOSHzRgAAAAABJRU5ErkJggg=="
    );
    (
        axum::response::AppendHeaders([(axum::http::header::CONTENT_TYPE, "image/png")]),
        base64::decode(png).unwrap(),
    )
}

// graceful shutdown ------------------------------------------------

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c");
    println!("Received Ctrl-C");
}
