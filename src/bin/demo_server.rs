use warp::Filter;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Sample {
    name: Option<String>,
    quest: Option<String>,
    captial_of_assyria: Option<String>,
    favorite_color: Option<String>,
    airspeed_velocity_of_an_unladen_swallow: Option<u32>
}

async fn respond(
    sample:Sample
    ) -> Result<impl warp::Reply, warp::Rejection> {
    // Some JSON input data as a &str. Maybe this comes from the user.

    Ok(warp::reply::json(&sample))
}

#[tokio::main]
async fn main() {



    // POST /sample
    let promote = warp::post()
        .and(warp::path("sample"))
        .and(warp::path::end())
        // Only accept bodies smaller than 16kb...
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and_then(respond);

    warp::serve(promote).run(([127, 0, 0, 1], 3030)).await
}

fn hello_there() -> &'static str {
    "Greetings"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_data() {
        let greeting = hello_there();
        assert_eq!("Greetings", greeting);
    }
}