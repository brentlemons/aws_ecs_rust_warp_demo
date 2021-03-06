use warp;

mod handlers;
mod routes;

#[tokio::main]
async fn main() {
    let aviation_routes = routes::aviation_routes();

    warp::serve(aviation_routes)
        .run(([127, 0, 0, 1], 3000))
        .await;        
}