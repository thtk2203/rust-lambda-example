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

    let service = create_service(shared_client.clone());
    let shared_service = Arc::new(service);

    run(service_fn(|event| {
        let service = shared_service.clone();
        println!("service cloned: {}", Arc::strong_count(&service));

        async move { service.handle_event(event).await }
    }))
    .await
}
