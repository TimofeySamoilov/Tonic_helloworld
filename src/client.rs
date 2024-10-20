use std::{thread, time};
use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;
use std::io;
pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let interval = time::Duration::from_secs(1);
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Please write a string!");

    loop {
        thread::sleep(interval);
        let request = tonic::Request::new(HelloRequest {name: name.clone(),});
        let response = client.say_hello(request).await?;
        println!("{:?}", response.into_inner().message);
    }

    Ok(())
}