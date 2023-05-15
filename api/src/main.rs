use axum::{
    routing::get,
    Router,
    response::Json
};

use serde::{Deserialize, Serialize};

//use serde_json::{Value, json};

#[derive(Serialize, Deserialize, Debug)]
struct Hello {
    hello: String,
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(root))
        .route("/foo", get(get_foo));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> Json<Hello> {
    let hello = Hello {
        hello: "world".to_owned()
    };
    Json(hello)
}

async fn get_foo() -> Json<Hello> {
    let hello = Hello {
        hello: "world".to_owned()
    };
    Json(hello)
}