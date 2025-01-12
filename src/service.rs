use crate::sample_client::{Client};
use crate::generic_handler::{function_handler, OutgoingMessage};
use lambda_runtime::{Error, LambdaEvent};
use serde_json::Value;
use std::future::Future;
use std::sync::Arc;

// pub(crate) fn create_handler(
//     shared_client: Arc<SampleClient>,
//     // event: LambdaEvent<Value>,
// ) -> impl Future<Output = Result<OutgoingMessage, Error>>
// {
//     let client = shared_client.clone();
//
//     async move |event| { function_handler(client, event).await }
// }

 pub(crate)  async fn run_handler(
    client: Arc<impl Client>,
    event: LambdaEvent<Value>,

) -> Result<OutgoingMessage, Error>
{
    function_handler(client, event).await
}