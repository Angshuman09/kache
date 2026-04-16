// phase-2 implementing ttl
use std::hash::Hash;
use std::collections::HashMap;
use std::time::{ Instant, Duration };
// use std::thread::sleep;
use std::io::{self, Write};

pub const BANNER: &str = r#"
$$\   $$\  $$$$$$\   $$$$$$\  $$\   $$\ $$$$$$$$\ 
$$ | $$  |$$  __$$\ $$  __$$\ $$ |  $$ |$$  _____|
$$ |$$  / $$ /  $$ |$$ /  \__|$$ |  $$ |$$ |      
$$$$$  /  $$$$$$$$ |$$ |      $$$$$$$$ |$$$$$\    
$$  $$<   $$  __$$ |$$ |      $$  __$$ |$$  __|   
$$ |\$$\  $$ |  $$ |$$ |  $$\ $$ |  $$ |$$ |      
$$ | \$$\ $$ |  $$ |\$$$$$$  |$$ |  $$ |$$$$$$$$\ 
\__|  \__|\__|  \__| \______/ \__|  \__|\________|
                                                                                          
"#;

#[derive(Debug)]
struct CacheEntry<V> {
    value: V,
    created_at: Instant,
    ttl: Duration,
}

impl<V> CacheEntry<V> {
    fn is_expired(&self) -> bool {
        self.created_at.elapsed() > self.ttl
    }
}

#[derive(Debug)]
struct Cache<K, V> {
    storage: HashMap<K, CacheEntry<V>>,
    default_ttl: Duration,
}

impl<K, V> Cache<K, V> where K: Eq + Hash {
    pub fn new(default_seconds: u64) -> Self {
        Cache {
            storage: HashMap::new(),
            default_ttl: Duration::from_secs(default_seconds),
        }
    }

    pub fn set(&mut self, key: K, value: V, ttl: Option<Duration>) {
        let expiration = ttl.unwrap_or(self.default_ttl);

        let entry = CacheEntry {
            value: value,
            created_at: Instant::now(),
            ttl: expiration,
        };

        self.storage.insert(key, entry);
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {

        let is_expired = if let Some(entry) = self.storage.get(key){
            entry.is_expired()
        }else{
            return None;
        };

        if is_expired{
            self.storage.remove(&key);
            None
        }else{
            self.storage.get(&key).map(|entry| &entry.value)
        }
    }

    pub fn delete(&mut self, key: K) -> Option<CacheEntry<V>> {
        self.storage.remove(&key)
    }

    pub fn exists(&mut self, key: &K) -> bool {
        let expired = match self.storage.get(key){
            Some(entry) => entry.is_expired(),
            None => return false
        };

        if expired{
            self.storage.remove(key);
            return false;
        }else{
            return true;
        }
    }

    pub fn size(&self) -> usize {
        self.storage.len()
    }
    pub fn clear(&mut self) {
        self.storage.clear()
    }
}
fn main() {
    // let mut kache = Cache::new(5);
    let pink = "\x1b[38;2;255;182;193m"; 
    let green = "\x1b[38;2;120;220;120m";
    let dark_red = "\x1b[38;2;160;40;40m";
    let purple = "\x1b[38;2;180;120;255m";
    let reset = "\x1b[0m";

    println!("{}{}{}",pink, BANNER, reset);
    println!("{} Commands: SET <key> <val>, GET <key>, DELETE <key>, EXISTS <key>, SIZE, EXIT", green);

    loop{
        print!("{}>", purple);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        println!("{:?}", parts);
        break;
    }

    // let short_ttl = Duration::from_secs(2);
    // kache.set("I", "love gooning", Some(short_ttl));

    // println!("{:?}", kache.get(&"I"));

    // sleep(Duration::from_secs(3));

    // match kache.get(&"temp_key"){
    //     Some(val)=> println!("Still here: {}", val),
    //     None=> println!("key is expired")
    // }

    // println!("{}",kache.size());
    // kache.clear();
}
