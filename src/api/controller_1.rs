use warp::Filter;

use crate::services;

pub fn hello_routes() -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    let hello_1 = warp::path!("hello_1" / String)    
        .map(|name| format!("Hello 1, {}, from the controller 1 and {}!", name, services::service_1::hello()));

    let hello_2 = warp::path!("hello_2" / String)    
        .map(|name| format!("Hello 2, {}, from the controller 1 and {}!", name, services::service_1::hello()));

    let routes = warp::path("controller_1")
        .and(hello_1
            .or(hello_2));
        

    routes
}