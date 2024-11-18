mod helper;
mod model;

use helper::json::{patch_struct, read_json, write_json};
use model::Scoreboard;
use warp::Filter;

#[tokio::main]
async fn main() {
    println!("ready to serve");

    let json_url = "static/state.json";
    warp::serve(
        warp::fs::dir("static").or(warp::path("api")
            .and(warp::path("control"))
            .and(warp::post())
            .and(warp::body::json())
            .map(|scoreboard: Scoreboard| {
                let mut old_scoreboard = read_json(json_url);
                patch_struct(&mut old_scoreboard, scoreboard).expect("Merge json failed");
                write_json(&old_scoreboard, json_url);

                warp::reply::with_status(
                    warp::reply::json(&serde_json::json!({"status": "Ok"})),
                    warp::http::StatusCode::OK,
                )
            })),
    )
    .run(([0, 0, 0, 0], 3030))
    .await;

    println!("Server is running on localhost:3030");
}
