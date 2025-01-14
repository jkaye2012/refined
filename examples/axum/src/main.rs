//! Demonstrates basic usage of `refined` with `axum`.

use axum::{extract, routing::post, Json, Router};
use axum_test::TestServer;
use rand::prelude::*;
use refined::{boundable::unsigned::ClosedInterval, Refinement};
use serde::{Deserialize, Serialize};
use serde_json::json;

type Name = Refinement<String, ClosedInterval<1, 255>>;

type SkillLevel = Refinement<u8, ClosedInterval<1, 10>>;

#[derive(Debug, Serialize, Deserialize)]
struct CreateSkier {
    name: Name,
    skill: SkillLevel,
}

#[derive(Debug, Serialize, Deserialize)]
struct Skier {
    id: u8,
    name: Name,
    skill: SkillLevel,
}

async fn create_skier(extract::Json(skier): extract::Json<CreateSkier>) -> Json<Skier> {
    let mut rng = rand::thread_rng();
    let id: u8 = rng.gen();

    Json(Skier {
        id,
        name: skier.name,
        skill: skier.skill,
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", post(create_skier));
    let server = TestServer::new(app).expect("Failed to build test server");
    println!("Test server running!\n");

    let good_request = json!({
        "name": "Candide",
        "skill": 10
    });
    println!("Sending good request: {}", &good_request);
    let successful_response = server.post("/").json(&good_request).await;
    println!(
        "Skier created successfully with id {}\n",
        successful_response.json::<Skier>().id
    );

    let bad_request = json!({
        "name": "Brian",
        "skill": 0
    });
    println!("Sending bad request (skill can't be 0): {}", &bad_request);
    let failed_response = server.post("/").json(&bad_request).await;
    println!("{}", failed_response.text());
}
