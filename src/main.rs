// phase-1
use std::hash::Hash;
use std::collections::HashMap;

#[derive(Debug)]
struct Cache <K, V>{
    storage: HashMap<K, V>
}

impl <K,V> Cache<K, V>
where K: Eq + Hash
{

    pub fn new() -> Self{
        Cache{
            storage: HashMap::new()
        }
    }
    
    pub fn set(&mut self, key: K, value: V){
        if self.storage.contains_key(&key){
            self.storage.insert(key, value);
            println!("Value updated");
        }else{
        self.storage.insert(key, value);
        }
    }

    pub fn get(&mut self, key:K)-> Option<&V>{
        self.storage.get(&key)
    }

    pub fn delete(&mut self, key: K)-> Option<V>{
        self.storage.remove(&key)
    }

    pub fn exists(&mut self, key: K)-> bool{
        self.storage.contains_key(&key)
    }

    pub fn size(&self)->usize{
        self.storage.len()
    }
    pub fn clear(&mut self){
        self.storage.clear()
    }
}
fn main(){

    let mut kache = Cache::new();

    kache.set("name1", "Wamiqa");
    kache.set("name2", "Angshu");
    kache.set("name3", "lawde");
    kache.set("name4", "bsdk");
    kache.set("name5", "mkc");

    match kache.get("name1"){
        Some(val) => println!("{:?}", val),
        None => println!("Key not found")
    }

    println!("{}", kache.exists("name1"));
    println!("{:?}", kache.size());

    kache.delete("name1");
    println!("{}", kache.exists("name1"));
    println!("{:?}", kache.size());

    match kache.get("name5"){
        Some(val) => println!("{}", val),
        None => println!("Value not found")
    }
    kache.set("name5", "kithe reh gaya");

    match  kache.get("name5") {
        Some(val)=> println!("{}", val),
        None => println!("Value not found")
    }

    kache.clear();

    println!("{:?}", kache.size());
}