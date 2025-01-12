use crate::generic_handler::{function_handler, OutgoingMessage};
use crate::sample_client::Client;
use lambda_runtime::{Error, LambdaEvent};
use serde_json::Value;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::runtime::Handle;
use tower::service_fn;
use tower::util::ServiceFn;

pub(crate) fn create_handler<T>(client: Arc<T>) -> impl Handler
where
    T: Client,
{
    SimpleHandler::<T>::new(client)
}

pub(crate) trait Handler: Send + Sync {
    async fn handle_event(&self, event: LambdaEvent<Value>) -> Result<OutgoingMessage, Error>;
}

pub(crate) struct SimpleHandler<T>
where
    T: Client,
{
    client: Arc<T>,
}

impl<T> SimpleHandler<T>
where
    T: Client,
{
    pub fn new(client: Arc<T>) -> Self {
        Self { client }
    }
}

impl<T> Handler for SimpleHandler<T>
where
    T: Client,
{
    async fn handle_event(&self, event: LambdaEvent<Value>) -> Result<OutgoingMessage, Error> {
        function_handler(self.client.clone(), event).await
    }
}

// pub(crate) fn create_handler(
//     shared_client: Arc<impl Client>,
//     // event: LambdaEvent<Value>,
// ) -> Box<dyn FnOnce(LambdaEvent<Value>) -> Result<OutgoingMessage, Error>> {
//     let f = |event| {
//         async move { run_handler(shared_client, event).await }
//     };
//
//     f
// }

// pub(crate) fn create_handler(
//     shared_client: Arc<impl Client>,
//     // event: LambdaEvent<Value>,
// ) ->impl FnMut(LambdaEvent<Value>) -> impl Future<Output=Result<OutgoingMessage, Error>> {
//     let f = |event| async move  {
//         run_handler(shared_client.clone(), event).await
//     };
//
//     f
// }

// pub(crate) fn create_handler(
//     shared_client: Arc<impl Client>,
//     // event: LambdaEvent<Value>,
// ) -> impl Future<Output = Result<OutgoingMessage, Error>> {
//     let f = |event|   {
//         async move { run_handler(shared_client, event).await }
//     };
//
//     f
// }

// pub(crate) fn create_handler(
//     sheared_client: Arc<impl Client>,
// ) -> impl FnMut(LambdaEvent<Value>) -> Pin<Box<dyn Future<Output = Result<OutgoingMessage, Error>> + 'static>> {
//     |event| {
//         let client = sheared_client.clone();
//         Box::pin(async move { run_handler(client, event).await })
//     }
// }

// pub(crate) fn create_handler(
//     sheared_client: Arc<impl Client>,
// ) -> impl FnMut(LambdaEvent<Value>) -> dyn Future<Output = Result<OutgoingMessage, Error>> {
//     |event| {
//         let client = sheared_client.clone();
//         Box::pin(async move { run_handler(client, event).await })
//     }
// }

// pub(crate) fn create_handler(
//     sheared_client: Arc<impl Client>,
// ) -> impl FnMut(LambdaEvent<Value>) -> Box<dyn Future<Output = Result<OutgoingMessage, Error>>> {
//     async move |event| {
//         let client = sheared_client.clone();
//         run_handler(client, event).await
//     }
// }

// pub(crate) fn create_handler<T, F>(
//     sheared_client: Arc<impl Client>,
// ) -> impl FnMut(LambdaEvent<Value>) -> F
// where
//     T: FnMut(LambdaEvent<Value>) -> F,
//     F: Future<Output = Result<OutgoingMessage, Error>>,
// {
//     |event| {
//         let client = sheared_client.clone();
//          async move { run_handler(client, event).await }
//      }
// }

// pub(crate) fn create_service<T, E>(sheared_client: Arc<impl Client>) -> ServiceFn<T>
// where
//     T: FnMut(LambdaEvent<Value>) -> E,
//     E: Future<Output =Result<OutgoingMessage, Error>>
// {
//     let handler = |event| {
//         let client = sheared_client.clone();
//         async move { run_handler(client, event).await }
//     };
//     let service = service_fn(handler);
//
//     service
// }

pub(crate) async fn run_handler(
    client: Arc<impl Client>,
    event: LambdaEvent<Value>,
) -> Result<OutgoingMessage, Error> {
    function_handler(client, event).await
}

//
// pub enum HandlerType {
//     Simple,
//     Multiple,
// }
//
// pub(crate) struct Handler<E>
// where
//     E: Handle,
// {
//     handler: E,
// }
//
// impl<E> Handler<E>
// where
//     E: Handle,
// {
//     pub fn new<T>(client: T, handler_type: HandlerType) -> Self where T: Client {
//         Self {
//             handler: match handler_type {
//                 HandlerType::Simple => SimpleHandler::<T>::new(client),
//                 HandlerType::Multiple => MultiHandler::<T>::new(client),
//             },
//         }
//     }
// }
//
// pub(crate) trait Handle: Send + Sync {
//     fn handle_event(&self, event: LambdaEvent<Value>);
// }
//
// pub(crate) struct SimpleHandler<T>
// where
//     T: Client,
// {
//     client: T,
// }
//
// impl<T> SimpleHandler<T>
// where
//     T: Client,
// {
//     pub fn new(client: T) -> Self {
//         Self { client }
//     }
// }
//
// impl<T> Handle for SimpleHandler<T>
// where
//     T: Client,
// {
//     fn handle_event(&self, event: LambdaEvent<Value>) {
//         self.client.invoke();
//     }
// }
//
// pub(crate) struct MultiHandler<T>
// where
//     T: Client,
// {
//     client: T,
// }
//
// impl<T> MultiHandler<T>
// where
//     T: Client,
// {
//     pub fn new(client: T) -> Self {
//         Self { client }
//     }
// }
//
// impl<T> Handle for MultiHandler<T>
// where
//     T: Client,
// {
//     fn handle_event(&self, event: LambdaEvent<Value>) {
//         self.client.invoke();
//     }
// }
