use warp::Filter;

mod api;
mod services;

#[tokio::main]
async fn main() {
    let health_route = warp::path!("health")
        .map(warp::reply);

    let routes = health_route
        .or( api::controller_1::hello_routes())
        .or( api::controller_2::hello_route())
        .with(warp::cors().allow_any_origin());

    warp::serve(routes)
        .run(([0, 0, 0, 0], 8080))
        .await;
}
