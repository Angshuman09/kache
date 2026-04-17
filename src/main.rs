mod cache;
mod parser;
use cache::Cache;
use parser::parser_input;
use std::time::{ Instant, Duration };
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
    let mut kache = Cache::new(30);
    let pink = "\x1b[38;2;255;182;193m";
    let green = "\x1b[38;2;120;220;120m";
    // let dark_red = "\x1b[38;2;160;40;40m";
    let purple = "\x1b[38;2;180;120;255m";
    let reset = "\x1b[0m";

    println!("{}{}{}", pink, BANNER, reset);
    println!("{} Commands: SET <key> <val>, GET <key>, DELETE <key>, EXISTS <key>, SIZE, EXIT", green);

    loop {
        print!("{}>", purple);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        parser_input(&input);

        let parts: Vec<String> = parser_input(&input);

        println!("{:?}", parts);

        if parts.is_empty() {
            continue;
        }

        let command = parts[0].to_uppercase();

        match command.as_str() {
            "SET" => {
                if parts.len() < 3 {
                    println!("(error) ERR wrong number of arguments for 'set' command");
                }
                if parts.len() == 3 {
                    kache.set(parts[1].to_string(), parts[2].to_string(), None);
                    println!("OK");
                } else {
                    println!("ERR: Usage: SET <key> <value>");
                }
            }

            "GET" => {
                if parts.len() < 2 {
                    println!("(error) ERR wrong number of arguments for 'get' command");
                }
                if let Some(key) = parts.get(1) {
                    match kache.get(&key.to_string()) {
                        Some(val) => println!("{}", val),
                        None => println!("there is no such value exists"),
                    }
                }
            }

            "DELETE" => {
                if parts.len() < 2 {
                    println!("(error) ERR wrong number of arguments for 'delete' command");
                }
                if let Some(key) = parts.get(1) {
                    kache.delete(&key.to_string());
                    println!("Ok");
                }
            }

            "EXISTS" => {
                if parts.len() < 2 {
                    println!("(error) ERR wrong number of arguments for 'exists' command");
                }
                if let Some(key) = parts.get(1) {
                    println!("{}", kache.exists(&key.to_string()));
                }
            }

            "SIZE" => {
                println!("{}", kache.size());
            }

            "CLEAN" =>{
               let remove_ele = kache.cleanup();

               println!("element removed: {}", remove_ele);
            }

            "CLEAR"=>{
                kache.clear();
            }

            "EXIT" => {
                println!("Byyy!");
                break;
            }

            _ => println!("ERR: Unknown command '{}'", command),
        }
    }
}
