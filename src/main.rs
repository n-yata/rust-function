use std::collections::HashMap;

use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};

use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Client,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct FunctRequest {
    #[serde(default)]
    url: String,
    #[serde(default)]
    method: String,
    #[serde(default)]
    headers: HashMap<String, String>,
    #[serde(default)]
    body: String,
}

#[derive(Serialize)]
struct FuncResponse {
    status: String,
    text: String,
}

async fn function_handler(event: LambdaEvent<FunctRequest>) -> Result<FuncResponse, Error> {
    let client = Client::new();

    let mut req_headers = HeaderMap::new();
    for (k, v) in event.payload.headers.iter() {
        req_headers.insert(
            HeaderName::from_bytes(k.as_bytes()).unwrap(),
            HeaderValue::from_str(v).unwrap(),
        );
    }

    let response = match event.payload.method.as_str() {
        "POST" => {
            client
                .post(&event.payload.url)
                .headers(req_headers)
                .body(event.payload.body.clone())
                .send()
                .await?
        }
        "GET" => {
            client
                .get(&event.payload.url)
                .headers(req_headers)
                .send()
                .await?
        }
        _ => {
            client
                .get(&event.payload.url)
                .headers(req_headers)
                .send()
                .await?
        }
    };

    let resp = FuncResponse {
        status: response.status().to_string(),
        text: response.text().await?,
    };

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
