use std::{thread, time};
use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;
use rdev::{listen, EventType};
use std::sync::{Arc, Mutex};
pub mod hello_world {
    tonic::include_proto!("helloworld");
}
#[warn(unreachable_code)]
async fn keyboard(s: Arc<Mutex<String>>) {
    listen (|event| {
        match event.event_type {
            EventType::KeyPress(key) => {
                println!("Key is pressed: {:?}", key);
                
                //s = key;
                if key == rdev::Key::Escape {
                    println!("Process is over");
                    std::process::exit(0);
                }
            }
            _ => ()
        }
    }).unwrap();
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let interval = time::Duration::from_secs(1);
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;
    let shared_string = Arc::new(Mutex::new(String::from("")));
    let string_clone = Arc::clone(&shared_string);
    let mut name = String::new();
    //io::stdin().read_line(&mut name).expect("Please write a string!");
    let f1 = tokio::spawn(async move {keyboard(shared_string).await});
    thread::sleep(interval);
    thread::sleep(interval);
    thread::sleep(interval);
    
    loop {
        
        thread::sleep(interval);
        let request = tonic::Request::new(HelloRequest {name: name.clone(),});
        let response = client.say_hello(request).await?;
        println!("{:?}", response.into_inner().message);
    }
    tokio::try_join!(f1).expect("problem");
    Ok(())
}