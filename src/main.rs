// fn main() {
//     println!("Hello, world!");
// }

use axum::Router;


#[tokio::main]
pub async fn main() {
    // Build our application by creating our router.
    let app = Router::new()
        .route( "/", axum::routing::get( || async {"Hello, World!"} ) 
    );

    // Run our application as a hyper server on http://localhost:3000.
    axum::Server::bind( &"0.0.0.0:3000".parse().unwrap())
        .serve( app.into_make_service() )
        .await
        .unwrap();
}