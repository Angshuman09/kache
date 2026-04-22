use std::fs::{OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::sync::{Arc, Mutex};

use crate::cache::Cache;
use crate::parser::parser_input;

pub fn log_command_to_disk(command_str: &str){
    let mut file = OpenOptions::new()
    .create(true)
    .append(true)
    .open("kache_log.aof")
    .expect("failed to open aof file");

    if let Err(e) = file.write_all(command_str.as_bytes()){
        println!("Failed to write to AOF: {}", e);
    }
}

pub fn restore_from_disk(kache: &Arc<Mutex<Cache<String, String>>>){
    println!("Loading data from disk");

    let file = match  OpenOptions::new().read(true).open("kache_log.aof"){
        Ok(f) => f,
        Err(_) => {
            println!("No existing AOF file found. Starting fresh.");
            return;
        }
    };

    let reader = BufReader::new(file);
    let mut cache = kache.lock().unwrap();
    let mut count = 0;

    for line in reader.lines(){
        if let Ok(command_str) = line{
            let parts = parser_input(&command_str);
            println!("{:?}", parts);
            if parts.is_empty(){ continue; }

            let command = parts[0].to_uppercase();

            match  command.as_str(){
                "SET" => {
                    if parts.len() == 3{
                        cache.set(parts[1].clone(), parts[2].clone(), None);
                        count += 1;
                    }
                },
                "DELETE"=>{
                    if parts.len() == 2{
                        cache.delete(&parts[1].clone());
                    }
                },
                _ => {}
            }
        }
    }

    println!("Succesfully restored {} keys from disk", count);
}
