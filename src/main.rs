use std::collections::HashMap;
use std::io::Error;
use std::fs::OpenOptions;
use serde_json::{
    from_reader,
    to_writer_pretty,
};

struct Todo {
    // use rust built in HashMap to store key - val pairs
    map: HashMap<String, bool>,
}

impl Todo {
    fn new() -> Result<Todo, Error> {
        let file = OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open("db.json")?;

        // serialize json has HashMap
        match from_reader(file) {
            Ok(map) => Ok(Todo { map }),
            Err(error) if error.is_eof() => Ok(Todo { map: HashMap:: new(), }),
            Err(error) => panic!("An error ocurred: {}", error),
        }
    }

    // "mut" makes a variable mutable.

    // "&" indicates a _reference_ (in Rust terms this is referred to as a "borrow" -> points to the location where it's stored in memory)
    // You can imagine the varaible as a pointer to the memory location where the value is stored, rather the being the "value" itself.
    fn insert(&mut self, key: String) {
        // insert a new item into our map.
        // we pass true as value
        self.map.insert(key, true);
    }

    fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        // open db.json
        let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("db.json")?;

        // write to file with serde
        to_writer_pretty(file, &self.map)?;
        Ok(())
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        // "get_mut" will give us a mutable reference to the value of key, or None if the value
        // is not present in the collection.
        match self.map.get_mut(key) {
            // We use "*" operator to de-reference the value and set it to false
            // isso seta o valor (direto no hash original) da chave como "false"
            Some(value) => Some(*value = false),
            None => None,
        }
    }
}

fn main() {
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify an item");

    let mut todo = Todo::new().expect("Initialisation of db failed");

    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("todo saved"),
            Err(why) => println!("An error occurred: {}", why),
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("'{}' is not present in the list", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("todo saved"),
                Err(why) => println!("An error occurred: {}", why),
            }
        }
    }
}
