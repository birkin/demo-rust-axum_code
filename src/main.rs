
use axum::{routing::get, Router};


#[tokio::main]
pub async fn main() {
    // Build our application by creating our router
    let app: Router = axum::Router::new()
        // .fallback( fallback.into_service() )
        .fallback( fallback )  // different from tutorial; whew
        .route( "/", get( hello )  
    );

    // Run our application as a hyper server on http://localhost:3000
    axum::Server::bind( &"0.0.0.0:3000".parse().unwrap())
        .serve( app.into_make_service() )
        .with_graceful_shutdown( shutdown_signal() )
        .await
        .unwrap();

    // // Run our application as a hyper server on http://localhost:3000
    // axum::Server::bind( &"0.0.0.0:3000".parse().unwrap())
    //     .serve( app.into_make_service() )
    //     .await
    //     .unwrap();

} // end main()

// route handlers ---------------------------------------------------

pub async fn fallback( uri: axum::http::Uri ) -> impl axum::response::IntoResponse {
    ( axum::http::StatusCode::NOT_FOUND, format!("No route {}", uri) )
}

pub async fn hello() -> String {
    "Hello, World!".into()
}

// graceful shutdown ------------------------------------------------

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect( "expect tokio signal ctrl-c" );
    println!( "Received Ctrl-C" );
}   