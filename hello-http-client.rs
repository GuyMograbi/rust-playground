// Define the service implementation
use tonic::{transport::Server, Request, Response, Status};

pub mod hello {
    tonic::include_proto!("hello"); // The generated Protobuf file
}


use hello::{hello_server::Hello, HelloRequest, HelloResponse};


#[derive(Default)]
pub struct MyHelloServer;

#[tonic::async_trait]
impl Hello for MyHelloServer {
    async fn say_hello(&self, request: Request<HelloRequest>) -> Result<Response<HelloResponse>, Status> {
        let name = request.into_inner().name;

        let response = HelloResponse {
            message: format!("Hello, {}!", name),
        };

        Ok(Response::new(response))
    }
}