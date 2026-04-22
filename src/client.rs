use std::{ io::{ Read, Write }, net::TcpStream, sync::{ Arc, Mutex } };
use crate::{ aof::log_command_to_disk, cache::Cache, parser::parser_input };

pub const BANNER: &str =
    r#"
$$\   $$\  $$$$$$\   $$$$$$\  $$\   $$\ $$$$$$$$\ 
$$ | $$  |$$  __$$\ $$  __$$\ $$ |  $$ |$$  _____|
$$ |$$  / $$ /  $$ |$$ /  \__|$$ |  $$ |$$ |      
$$$$$  /  $$$$$$$$ |$$ |      $$$$$$$$ |$$$$$\    
$$  $$<   $$  __$$ |$$ |      $$  __$$ |$$  __|   
$$ |\$$\  $$ |  $$ |$$ |  $$\ $$ |  $$ |$$ |      
$$ | \$$\ $$ |  $$ |\$$$$$$  |$$ |  $$ |$$$$$$$$\ 
\__|  \__|\__|  \__| \______/ \__|  \__|\________|
                                                                                          
"#;

pub fn handle_client(mut stream: TcpStream, kache: Arc<Mutex<Cache<String, String>>>) {
    let mut buffer = [0; 1024];
    // let pink = "\x1b[38;2;255;182;193m";

    let pink = "\x1b[38;2;255;182;193m";
    let green = "\x1b[38;2;120;220;120m";
    // let dark_red = "\x1b[38;2;160;40;40m";
    let purple = "\x1b[38;2;180;120;255m";
    let reset = "\x1b[0m";

    let des = "Kache : in-memory cache system";
    let welcome_msg = format!(
        "{}{}{}\n{}{des}{}\n{}Commands: SET, GET, DELETE, EXISTS, SIZE, CLEAN, EXIT{}\n\n{}kache > {}",
        pink, BANNER, reset, green, reset, pink, reset, purple, reset // Add the purple prompt at the very end
    );

    // 2. Send it immediately to the client
    if let Err(e) = stream.write_all(welcome_msg.as_bytes()) {
        println!("Failed to send banner to client: {}", e);
        return;
    }

    loop {
        let bytes_read = match stream.read(&mut buffer) {
            Ok(0) => {
                break;
            }
            Ok(n) => n,
            Err(_) => {
                break;
            }
        };

        println!("{}", bytes_read);

        let input = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();

        let parts = parser_input(&input);

        if parts.is_empty() {
            continue;
        }

        let command = parts[0].to_uppercase();

        let mut response = String::new();

        let mut cache = kache.lock().unwrap();

        match command.as_str() {
            "SET" => {
                if parts.len() == 3 {
                    cache.set(parts[1].to_string(), parts[2].to_string(), None);
                    let log_str = format!("SET {} {}\n", parts[1], parts[2]);
                    log_command_to_disk(&log_str);
                    response = "OK\n".to_string();
                } else {
                    response = "ERR wrong number of arguments for 'set'\n".to_string();
                }
            }

            "GET" => {
                if let Some(key) = parts.get(1) {
                    match cache.get(&key.to_string()) {
                        Some(val) => {
                            response = format!("{}\n", val);
                        }
                        None => {
                            response = "(nil)\n".to_string();
                        }
                    }
                } else {
                    response = "ERR wrong number of arguments for 'get'\n".to_string();
                }
            }

            "DELETE" => {
                if let Some(key) = parts.get(1) {
                    cache.delete(key);
                    let log_str = format!("DELETE {}\n", key);
                    log_command_to_disk(&log_str);
                    response = "OK\n".to_string();
                }
            }

            "EXISTS" => {
                if parts.len() < 2 {
                    response =
                        "{}(error) ERR wrong number of arguments for 'exists' command".to_string();
                }
                if let Some(key) = parts.get(1) {
                    response = format!("{}\n", cache.exists(&key.to_string()));
                }
            }

            "SIZE" => {
                response = format!("{}\n", cache.size());
            }

            "CLEAN" => {
                let remove_ele = cache.cleanup();

                response = format!("Elements removed: {}\n", remove_ele);
            }

            "CLEAR" => {
                cache.clear();
            }

            "EXIT" => {
                let _ = stream.write_all(b"Bye!\n");
                break;
            }

            _ => {
                response = format!("ERR Unknown command '{}'\n", command);
            }
        }

        response.push_str(&format!("{}kache > {}", purple, reset));

        if let Err(e) = stream.write_all(response.as_bytes()) {
            println!("Failed to send response: {}", e);
            break;
        }
    }
}
