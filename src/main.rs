mod cache;
mod parser;
use cache::Cache;
use parser::parser_input;
use std::io::{ self, Write };

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

fn main() {
    let mut kache = Cache::new(30, 3);
    let pink = "\x1b[38;2;255;182;193m";
    // let green = "\x1b[38;2;120;220;120m";
    // let dark_red = "\x1b[38;2;160;40;40m";
    let purple = "\x1b[38;2;180;120;255m";
    let reset = "\x1b[0m";

    println!("{}{}{}", pink, BANNER, reset);
    println!("{} Kache : in-memory cache system", pink);

    loop {
        print!("{}kache > ", purple);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        parser_input(&input);

        let parts: Vec<String> = parser_input(&input);

        // println!("{:?}", parts);

        if parts.is_empty() {
            continue;
        }

        let command = parts[0].to_uppercase();

        match command.as_str() {
            "SET" => {
                if parts.len() < 3 {
                    println!("{}(error) ERR wrong number of arguments for 'set' command", pink);
                }
                if parts.len() == 3 {
                    kache.set(parts[1].to_string(), parts[2].to_string(), None);
                    println!("{}OK", pink);
                } else {
                    println!("{}ERR: Usage: SET <key> <value>", pink);
                }
            }

            "GET" => {
                if parts.len() < 2 {
                    println!("{}(error) ERR wrong number of arguments for 'get' command", pink);
                }
                if let Some(key) = parts.get(1) {
                    match kache.get(&key.to_string()) {
                        Some(val) => println!("{}{}",pink, val),
                        None => println!("{}there is no such value exists",pink),
                    }
                }
            }

            "DELETE" => {
                if parts.len() < 2 {
                    println!("{}(error) ERR wrong number of arguments for 'delete' command",pink);
                }
                if let Some(key) = parts.get(1) {
                    kache.delete(&key.to_string());
                    println!("{}Ok", pink);
                }
            }

            "EXISTS" => {
                if parts.len() < 2 {
                    println!("{}(error) ERR wrong number of arguments for 'exists' command",pink);
                }
                if let Some(key) = parts.get(1) {
                    println!("{}{}",pink, kache.exists(&key.to_string()));
                }
            }

            "SIZE" => {
                println!("{}{}",pink, kache.size());
            }

            "CLEAN" => {
                let remove_ele = kache.cleanup();

                println!("{}element removed: {}",pink, remove_ele);
            }

            "CLEAR" => {
                kache.clear();
            }

            "EXIT" => {
                println!("{}Byyy!",pink);
                break;
            }

            _ => println!("{}ERR: Unknown command '{}'",pink, command),
        }
    }
}