use std::collections::HashMap;

use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};

use reqwest::{
    header::{HeaderMap, HeaderValue, CONTENT_TYPE},
    Client,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {
    zipcode: String,
}

#[derive(Serialize)]
struct Response {
    status: String,
    text: String,
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
/// - https://github.com/aws-samples/serverless-rust-demo/
async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    // Extract some useful info from the request
    let zipcode = event.payload.zipcode;

    let client = Client::new();

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let mut params = HashMap::new();
    params.insert("zipcode", zipcode);

    // GETリクエストを送信
    let response: reqwest::Response = client
        .get("https://zipcloud.ibsnet.co.jp/api/search")
        .headers(headers)
        .query(&params)
        .send()
        .await?;

    // Prepare the response
    let resp = Response {
        status: response.status().to_string(),
        text: response.text().await?,
    };

    // Return `Response` (it will be serialized to JSON automatically by the runtime)
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
