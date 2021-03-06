use std::convert::Infallible;

use warp::{self};

/// Returns a list of numbers as JSON
///
pub async fn list_numbers() -> Result<impl warp::Reply, Infallible> {
    let numbers: Vec<i32> = (0..16).collect();
    Ok(warp::reply::json(&numbers))
}