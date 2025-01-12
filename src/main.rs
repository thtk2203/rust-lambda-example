use lambda_runtime::{run, service_fn, tracing, Error};
use std::sync::Arc;

mod sample_client;
use sample_client::{Client, SampleClient};
mod service;
use service::*;
mod generic_handler;
use generic_handler::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let client = SampleClient::new();
    let sheared_client = Arc::new(client);

    run(service_fn(|event| {
        let client = sheared_client.clone();

        async move {
            run_handler(client, event).await
        }
    })).await
}
