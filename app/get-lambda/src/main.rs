mod models;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use models::ReadModel;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

/// Main function handler.  This is executed when a request comes in via the Function URL
async fn function_handler(mut event: LambdaEvent<ReadModel>) -> Result<ReadModel, Error> {
    event.payload.updated_timestamp = chrono::Utc::now();
    Ok(event.payload)
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

    lambda_runtime::run(service_fn(function_handler)).await
}
