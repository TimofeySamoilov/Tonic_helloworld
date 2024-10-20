use std::{fmt::Debug, string, thread, time};
use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;
use rdev::{listen, EventType};
use std::sync::{Arc, Mutex};
pub mod hello_world { tonic::include_proto!("helloworld"); }
#[warn(unreachable_code)]


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let interval = time::Duration::from_millis(32);
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;
    let shared_string = Arc::new(Mutex::new(String::from("")));
    let string_clone = Arc::clone(&shared_string);
    let mut name = String::new();
    //io::stdin().read_line(&mut name).expect("Please write a string!");
    let f = tokio::spawn(keyboard(shared_string));
    
    loop {
        name = format!("{:?}", string_clone).to_string();
        thread::sleep(interval);
        let request = tonic::Request::new(HelloRequest {name: name.clone(),});
        let response = client.say_hello(request).await?;
        println!("{:?}", response.into_inner().message);

    }
    tokio::try_join!(f).expect("problem");
    Ok(())
}
async fn keyboard(shared_string: Arc<Mutex<String>>) {
    listen (move |event| {
        match event.event_type {
            EventType::KeyPress(key) => {
                //println!("Key is pressed: {:?}", key);
                let mut s = shared_string.lock().unwrap();
                *s = format!("{:?}", key);
                if key == rdev::Key::Escape {
                    println!("Process is over");
                    std::process::exit(0);
                }
            }
            _ => { let mut s = shared_string.lock().unwrap();
            *s = "No one is pressed".to_string(); }
        }
    }).unwrap();
}