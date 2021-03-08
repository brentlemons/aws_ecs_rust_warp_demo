use std::convert::Infallible;

use geo::prelude::*;
use geo::point;

use warp::{self};

/// Returns a list of numbers as JSON
///
pub async fn list_numbers() -> Result<impl warp::Reply, Infallible> {
    let numbers: Vec<i32> = (0..16).collect();
    Ok(warp::reply::json(&numbers))
}

pub async fn to_radians() -> Result<impl warp::Reply, Infallible> {
    // New York City
    let p1 = point!(x: -74.006f64, y: 40.7128f64);

    // London
    let p2 = point!(x: -0.1278f64, y: 51.5074f64);

    let distance = p1.haversine_distance(&p2);
    
    Ok(warp::reply::json(&distance))
}