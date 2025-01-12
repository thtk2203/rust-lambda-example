use lambda_runtime::{run, service_fn, tracing, Error};
use std::sync::Arc;

mod sample_client;
use sample_client::*;
mod service;
use service::*;
mod generic_handler;
use generic_handler::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let client = SampleClient::new();
    let shared_client = Arc::new(client);

    let handler = create_handler(shared_client.clone());
    let shared_handler = Arc::new(handler);

    let handle = |event| {
        let handler = shared_handler.clone();

        async move { handler.handle_event(event).await }
    };

    run(service_fn(handle)).await
}
