use axum::{http::StatusCode, response::Html, routing::get, Router};
use image::ImageOutputFormat; // for image-to-base64 work
                              // use std::{arch::aarch64::int32x2_t, io::Cursor}; // for image-to-base64 work
use std::io::Cursor; // for image-to-base64 work

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
        .route("/items/:id", get(get_items_id));

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

// pub async fn get_demo_png() -> impl axum::response::IntoResponse {
//     // let png = concat!(
//     //     "iVBORw0KGgoAAAANSUhEUgAAAAEAAAAB",
//     //     "CAYAAAAfFcSJAAAADUlEQVR42mPk+89Q",
//     //     "DwADvgGOSHzRgAAAAABJRU5ErkJggg=="
//     // );
//     let png = "iVBORw0KGgoAAAANSUhEUgAAAC4AAAAfCAYAAACYn/8/AAAJZklEQVR4Ae2Xf2xeVRnHP+fe9977du22bh1r6xy4BhwDOoYbKohxmDgQ+mNiSgwKKEMCEYMRFQ3ENDGYmBBNJCSgbERB0VY2aSdsRGQqAQdMkG0MEZiuhTFWNtZu7b3vfd97/J7blbx0TGQC+ocn57nn5/M83+c5z3nO+/L/Ig/YLnxH6mK78exSCq7vyPUtGP5XigPqADmawKS5sKrvWxkxMXatlUGW/6IRk5XvO4uZdjn1qMTn0pJ00KVuXu15NFsHWKeST+hjOXLwHkdQrDxoBULnbm0HjeoXnZgw5Makwu9c3xquDSN65P2msXY+npQYSGbQYnqp5EZ0UZPzS47lrRsgXt5ScUrEpGacLW5nvbXUeT6X24zfRgVmJylbBOV9kUddnPGsdtYWp9BcGuPbZGyRUb0SsKymn9xIrf/LauUot8F0k7nWkXGfCZJ3fHZjzAbKE3MTrQXDUiKtxXEHZxtLbdTPXXEbz0QRxyUJaI8qWdHHl+fduFz0KJQ1K3Iy/cBg/AjShEuDhF+kESuSMndOvZfd9pOSfy+SxGvFXkZgfkTqJiTG0C0c3ch2pNBZtBXjjlFDqjfnY7SmbRwsY238RB68KB7jMSwLjaEgrxstO1JDRR9P5MbOS651pCkqvsGvWLZjqC0GzBasBaafp1GxIHGogccWEyzZpFWkXHfErGanunktdINnunHC0WVaGJZ5WRa+ZBV7Rp63aB0y28HUxHJaVOShNOYV5L/IsKSkDQKdC6v6+FV9Z8DEULtz0JnCaJ5DGJfY7/kESTsnGpgiAx7VJrNhKf6SDaTuPpRS7krL3Cohq9yljxQXXrdAJedyknVehxMSn+fTTk43DrTLAF0UxIAAHm1gfRwzKG+t0GUjyZAt495xe/4Nkoh8lyfeclyhrIm6rMJvwgJbJPfqfLWL4pnS70DKsM269Kdpzw635nl8S1iWmb3Lqa+psF0+75IHtxExqM2ZjnNJ2MfjlnGPOybF87Yo4PhEEad5VYybfzuoOB73l4RruW1CXtzO+ihkmfCkJuP4aA4DlZcolTO+XKjNOD+oo760n8/SwBXxELuKRRqTGB0GnUkHLXHGNwRxn5BO15Gh1oWWx1sqzkZxvjFPOUspZIbOuI0Patef04DVpJwqkmrwfZpLL/K9sAbKo+xwl2rGwXv8+XQPD0vuc0LWqPY4kau+UtwXXcDIGHTrJJfDgJ6YdnY51mpya27esVfP5/1CKcNlo06mgJx4+9STWRk/JnUGrNZ0pXqLIc1KCC8UU+73JOpOxetIWIBKxg0S0yJC+WaPa4t9/DWpsBJx6hLGmpMofd+oFmZjgkasfGTz9fGt1lkdNGEKs5AMrYzPq1NdTVyhEo9QMhW+b7rJdHnvxs+32KJHMxXwDF8y93HAuOm0nTOE69dKTQ2Kp7LaQppypfQPZ5bPyIhTZFezcrHsxLnOsb2OrJkCO0bRkVM8NqBiy1TEbLwQshKZkp2ZBWFzPaX0VYxxqm21DDfIJwXuERn4ZGZY5Vluy+9VOTfk4qiPOx5Qxst/wQX9PFgus6BUYpWOo6AH4dlwLTdl8OGojnP02DjQTvChoDVrg/diBkYJr1nJk9fdwc19KQdoxC9Mw1ZK2ATsTX3cc9532Xjfq0ThUfK8GKthgwONSiUM+JCctUKPVb12zYrL7Iw8FkUCbQXaZZx8s1WiN5tIxYR+V/xUN/zCOOF6ef5mncDvQ48WF4NaPwS4NRGF4Cj2rR5k5sgw+6Mapkvz7YtCLpjfQmXX02ydupBFfX/h4c1bOH1hK3vOez91pWek0NPxZRL7WrXq2UjTioCbs4yPKWcvULjMNasZtEspmg24cHXBBw60nvtQL2dJvx8uStqwAn+twH/BwhTlV3kIYwyHlKySECiV7Z4N/avXMLsm/73FU6UGSmMvEE2DJx5/ksd7ehnZtjXnH9y3h9ZaXUJdKqpkSpdTYXSnUs1frkFagoWRA91FjfCN5QL0qWLTcerB0aKugIS2c0MQcrXjlIA8XBWymWKvIKGuih09qRB4sL8yhVPvGeW5fDbg1tYynzvG4nkBjw6lfOShfIH5s2Zw/yl7adJvw0QeUTzrMmtNqH2PVGM9IWLTlE5Zl4ELi/2s1bJgjG/V0mv6XT//12K6yeI2ztHKzxXb0w7oUuwv4zfJkYGPA+r0GeksyBAvl2ZBx8tAuYnenR7Tyy9y/lyo14vmjgo81u+dw/MjMctm7OaYWkNJzEZaPQ/32JU9N8gI98rFw2VKM0PC2gIUdLeV9a4S+B9Wh7SWxK0q3Y5VPRCYi5Xop6djjAwlXHf/INfPm8auObXUHVtHY1Fczi2AQhGFIibN8OYGL5mvzMUUJEnGsnHIGgekIczsWTMHsPU4l9mytZlkKGlgFC2BnvNwdwybX+XlnaPcOJpy8UkN/PGMJpaKYZ5OeTxEWjBsIi9e/tVHuixbtaB+WOGqZJS/BTVMPW4Gw2MpVw6NMdb6Bz7Qs51vrhvkT48Mkbx4gEJcJlTKCpRnddLG842RKIwuldk2DD/7B9QFkusZo1MyAuH5UBgrEQ6MEDy4i3LfDh5a83e+dtYmmhtCHin6PPPRJm4xBebFKetq1vJji2T06j4zXpyS8d7Br1W6MfqBM9rJXFPhCWWUmd7d+HecxC31ARvbn+BWt/UCOObUFhY3RJxSF3JCXUEREDBL4TRdPEWFTiATfIG1vqGyP6WUZBwQkFcUCgPDCU/tS3h0+xAbfzDMs06mo55W1vRu5us9y7lC3v4qIbNNL7uVPHy1FbfnsKRNoVscbWOOfntX9ECtceNftrJq3WKaXf8wVNT8UcvhfZ+G46+cQes1jbSeDfPPgKO1Vi8yotdV5yw38atWzlk5nxWuL7221MElrj+Bx/XflCY2K68fLSGp7UROprb/ZIQBbllM0K3Tce0Dat9UYNWGni78Cb5u8By55bsX8QnXph18RzrXub7txnPtZDKTJ6rH25Xw5ynhJ+eyQC9ZTwRnm7W84BSf38shx+YAbAVzooil0Dzingh4z1TsCRuwT+ESlwJALZPKAzL+TIWo7cD9YemJLJc6XVbzRvOTtr/5UJ7XD0lwfy7iDi5zHPYwXnBrR0oWXT4gbeO0ciefUhfpzkPW9Y+IrKx2jC93UWffAdBO9gQJbO4oN7Zg+E+LfYcBT8ZneRtAc7C8rcIOypzcvBs6Jut8V8f/BO3Oxfc3lWsOAAAAAElFTkSuQmCC";
//     (
//         axum::response::AppendHeaders([(axum::http::header::CONTENT_TYPE, "image/png")]),
//         base64::decode(png).unwrap(),
//     )
// }

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

// graceful shutdown ------------------------------------------------

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c");
    println!("Received Ctrl-C");
}
