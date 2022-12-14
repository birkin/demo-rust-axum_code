use axum::{http::StatusCode, response::Html, routing::get, Router};
use axum_macros::debug_handler; // for debugging; recommended by <https://docs.rs/axum-macros/latest/axum_macros/attr.debug_handler.html>
use image::ImageOutputFormat; // for image-to-base64 work
use serde_json::{json, Value};
use std::collections::HashMap; // for query-params work
use std::io::Cursor; // for image-to-base64 work
use axum::Json;

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
        .route("/demo_tutorial.png", get(get_demo_tutorial_png))
        .route("/demo_direct.png", get(get_demo_direct_png))
        .route(
            "/verb_foo",
            get(verb_foo_get)
                .post(verb_foo_post)
                .put(verb_foo_put)
                .patch(verb_foo_patch)
                .delete(verb_foo_delete),
        )
        .route("/items/:id", get(get_items_id)) // demonstrates path-parameters
        .route("/items_query_params_example_A", get(get_items_tutorial)) // demonstrates query-parameters
        .route("/items_query_params_example_B", get(get_items_birkin)) // demonstrates query-parameters, with more explicit steps
        .route(
            "/demo.json",
            get(get_demo_json) // demonstrates basic json get-response
                .put(put_demo_json), // demonstrates json-handling via tutorial
        )
        .route(
            "/demo_birkin.json",
            get(get_demo_json_birkin) // hack; returns a `405 Method Not Allowed`
                .put(put_demo_json_birkin), // demonstrates PUT handling with more explicit steps
        );

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

pub async fn get_demo_tutorial_png() -> impl axum::response::IntoResponse {
    /*  The tutorial provides a base64-string representing a single invisible pixel, which it then decodes and sends.
        I wanted to see an actual image, so I first figured out how to
            load and convert an image to a base64-encoded string.
        The get_demo_direct_png() function below returns the loaded object more directly.
    */

    // let img = concat!(
    //     "iVBORw0KGgoAAAANSUhEUgAAAAEAAAAB",
    //     "CAYAAAAfFcSJAAAADUlEQVR42mPk+89Q",
    //     "DwADvgGOSHzRgAAAAABJRU5ErkJggg=="
    // );

    // Load the image file
    let filepath = "./src/ferris_image/happy_ferris.png".to_string();
    let image_obj: image::DynamicImage = image::open(filepath).unwrap();

    // Convert image to a base64-encoded string
    let mut image_data: Vec<u8> = Vec::new();
    image_obj
        .write_to(&mut Cursor::new(&mut image_data), ImageOutputFormat::Png)
        .unwrap();
    let image_as_base64_string = base64::encode(image_data);

    // convert the base64-encoded string to a vector of bytes
    let img = base64::decode(image_as_base64_string).unwrap();

    // prepare the image-header
    let image_header =
        axum::response::AppendHeaders([(axum::http::header::CONTENT_TYPE, "image/png")]);

    // return the image
    (image_header, img)
}

pub async fn get_demo_direct_png() -> impl axum::response::IntoResponse {
    /*  The tutorial provides a base64-string representing a single invisible pixel, which it then decodes and sends.
    This function returns the loaded image more directly. */

    // Load the image file
    let filepath = "./src/ferris_image/happy_ferris.png".to_string();
    let image_obj: image::DynamicImage = image::open(filepath).unwrap();

    // convert the image to a vector of bytes
    let mut img: Vec<u8> = Vec::new();
    image_obj
        .write_to(&mut Cursor::new(&mut img), ImageOutputFormat::Png)
        .unwrap();

    // prepare the image-header
    let image_header =
        axum::response::AppendHeaders([(axum::http::header::CONTENT_TYPE, "image/png")]);

    // return the image
    (image_header, img)
}

pub async fn verb_foo_get() -> String {
    "GET verb_foo\n".to_string()
}

pub async fn verb_foo_post() -> String {
    "POST verb_foo\n".to_string()
}

pub async fn verb_foo_put() -> String {
    "PUT verb_foo\n".to_string()
}

pub async fn verb_foo_patch() -> String {
    "PATCH verb_foo\n".to_string()
}

pub async fn verb_foo_delete() -> String {
    "DELETE verb_foo\n".to_string()
}

pub async fn get_items_id(axum::extract::Path(id): axum::extract::Path<String>) -> String {
    format!("GET items with path id, ``{:?}``\n", id)
}

pub async fn get_items_tutorial(
    axum::extract::Query(query_params): axum::extract::Query<HashMap<String, String>>,
) -> String {
    format!("GET items with query params, ``{:?}``\n", query_params) // returns: --> GET items with query params, ``{"foo": "bar"}`` <--
}

#[debug_handler]
pub async fn get_items_birkin(
    query_params: axum::extract::Query<HashMap<String, String>>,
) -> String {
    let extract_params: HashMap<String, String> = query_params.0; // don't exactly understand why this works, but it does. It's not as if the Query object has a .1, .2, etc. It has lots of other methods.
    format!(
        "GET items with query params (with more-explicit handling), ``{:?}``\n",
        extract_params
    )
}

pub async fn get_demo_json() -> axum::extract::Json<Value> {
    json!( {"a":"b"} ).into()
}

pub async fn get_demo_json_birkin() -> impl axum::response::IntoResponse {
    (
        axum::http::StatusCode::METHOD_NOT_ALLOWED,
        format!("405 Method Not Allowed"),
    )
}

pub async fn put_demo_json(
    axum::extract::Json(data): axum::extract::Json<serde_json::Value>,
) -> String {
    format!("PUT demo_json with data, ``{:?}``\n", data) // yields: PUT demo_json with data, ``Object {"a": String("b")}``
}

#[debug_handler]
pub async fn put_demo_json_birkin(data: String) -> String {
    println!("data = ``{:?}``", data); // yields: data = ``"{\"a\":\"b\"}"``
    let put_data: Json<HashMap<String, String>> = axum::extract::Json(serde_json::from_str(&data).unwrap());
    format!(
        "PUT demo_json with data (with more-explicit handling), ``{:?}``\n",
        put_data
    )  // not quite (though this would be workable, I think); yields: PUT demo_json with data (with more-explicit handling), ``Json({"a": "b"})``
}

// #[debug_handler]
// pub async fn put_demo_json_birkin(data: String) -> String {
//     println!( "data = ``{:?}``", data );  // yields: data = ``"{\"a\":\"b\"}"``
//     let put_data: HashMap<String, String> = serde_json::from_str(&data).unwrap();
//     format!(
//         "PUT demo_json with data (with more-explicit handling), ``{:?}``\n",
//         put_data
//     )  // not quite; yields: PUT demo_json with data (with more-explicit handling), ``{"a": "b"}``
// }

// #[debug_handler]
// pub async fn put_demo_json_birkin(data: String) -> String {
//     println!( "data = ``{:?}``", data );  // yields: data = ``"{\"a\":\"b\"}"``
//     let put_data: axum::extract::Json<serde_json::Value> =
//         axum::extract::Json(serde_json::from_str(&data).unwrap());
//     format!(
//         "PUT demo_json with data (with more-explicit handling), ``{:?}``\n",
//         put_data
//     )  // not quite; yields: PUT demo_json with data (with more-explicit handling), ``Json(Object {"a": String("b")})``
// }

// #[debug_handler]
// pub async fn put_demo_json_birkin(data: axum::extract::Query<HashMap<String, String>>,) -> String {
//     println!( "data = ``{:?}``", data );
//     let put_data: HashMap<String, String> = axum::extract::Json(data.0).0;
//     format!(
//         "PUT demo_json with data (with more-excplicit handling), ``{:?}``\n",
//         put_data
//     )  // nope; returns "PUT demo_json with data (with more-explicit handling), ``{}``"
// }

// graceful shutdown ------------------------------------------------

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c");
    println!("Received Ctrl-C");
}
