use warp::Filter;

use crate::services;

pub fn hello_route() -> impl Filter<Extract=impl warp::Reply, Error=warp::Rejection> + Clone {
    let hello = warp::path!("hello_2" / String)    
        .map(|name| format!("Hello, {}, from the controller 2 and {}!", name, services::service_2::hello()));

    hello
}