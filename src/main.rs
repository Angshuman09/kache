mod cache;
mod parser;
mod client;
use cache::Cache;
use std::sync::{Mutex, Arc};
use std::net::{TcpListener};
use std::thread;

use crate::client::handle_client;



fn main() {

    let kache = Arc::new(Mutex::new(Cache::new(30, 3)));

    let listener = TcpListener::bind("127.0.0.1:6969").unwrap();
    println!("Server listening on the port 7878..");

    for stream in listener.incoming(){
        match stream {
            Ok(stream)=>{
                println!("New client connected!");
                let kache_handle = Arc::clone(&kache);

                thread::spawn(move ||{
                    handle_client(stream, kache_handle);
                    println!("Client disconnected");
                });
            }

            Err(e)=>{
                println!("Connection failed: {}", e);
            }
        }
    }

    
}
