use tonic::{transport::Server, Request, Response, Status};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest};
use chrono::prelude::*;
pub mod hello_world {
    tonic::include_proto!("helloworld"); // The string specified here must match the proto package name
}
#[warn(unused_variables)]
#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello( &self, request: Request<HelloRequest>, ) -> Result<Response<HelloReply>, Status> { // Return an instance of type HelloReply

        println!("{:?}", request.into_inner().name);
        let now: DateTime<Utc> = Utc::now();
        let reply = HelloReply {
            message: format!("{}", now.format("%Y-%m-%d_%H:%M:%S")), // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    Server::builder().add_service(GreeterServer::new(greeter)).serve(addr).await?;

    Ok(())
}