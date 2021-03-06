// use std::convert::Infallible;
use warp::{self, Filter};

use crate::handlers;

/// All aviation routes
pub fn aviation_routes(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    numbers_list()
        .or(to_radians())
}

/// GET /numbers
fn numbers_list(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("numbers")
        .and(warp::get())
        .and_then(handlers::list_numbers)
}

fn to_radians(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("radians" / String)
        .and(warp::get())
        .and_then(handlers::to_radians)
}