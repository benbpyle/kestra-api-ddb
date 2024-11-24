mod data;
mod models;

use aws_sdk_dynamodb::Client;
use data::create_item;
use lambda_http::http::StatusCode;
use lambda_http::{run, service_fn, Error, IntoResponse, Request, Response};
use models::WriteModel;
use serde_json::json;
use shared::clients::clients::new_client;
use tracing::info;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

/// Main function handler.  This is executed when a request comes in via the Function URL
async fn function_handler(
    table_name: &str,
    client: &Client,
    event: Request,
) -> Result<impl IntoResponse, Error> {
    let status_code = StatusCode::OK;
    let return_body = "Success";

    // extra and parse the body of the request into a MomentoPayload
    let body = event.body();
    let body_string = std::str::from_utf8(body).expect("Body wasn't supplied");
    let payload: Result<WriteModel, serde_json::Error> = serde_json::from_str(body_string);
    let extract = payload.unwrap();
    let read = create_item(client, table_name, extract).await;
    // the final output response sent back to the client
    let response = Response::builder()
        .status(status_code)
        .header("Content-Type", "application/json")
        .body(json!(read).to_string())
        .map_err(Box::new)?;

    info!(body = return_body, "Output of request={:?}", status_code);
    Ok(response)
}

/// Main function which is the starting point for the Lambda
#[tokio::main]
async fn main() -> Result<(), Error> {
    let filtered_layer = tracing_subscriber::fmt::layer()
        .pretty()
        .json()
        .with_target(true)
        .with_file(true);

    tracing_subscriber::registry()
        .with(filtered_layer)
        .with(EnvFilter::from_default_env())
        .init();

    let client = new_client("FALSE".to_string()).await;
    let table_name = &std::env::var("TABLE_NAME").expect("TABLE_NAME must be set");
    let shared_client = &client;

    run(service_fn(move |event: Request| async move {
        function_handler(table_name, shared_client, event).await
    }))
    .await
}
