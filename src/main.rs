use std::{collections::HashMap, io::Error, fs::write};

struct Todo {
    // use rust built in HashMap to store key - val pairs
    map: HashMap<String, bool>,
}

impl Todo {
    // "mut" makes a variable mutable.

    // "&" indicates a _reference_ (in Rust terms this is referred to as a "borrow" -> points to the location where it's stored in memory)
    // You can imagine the varaible as a pointer to the memory location where the value is stored, rather the being the "value" itself.
    fn insert(&mut self, key: String) {
        // insert a new item into our map.
        // we pass true as value
        self.map.insert(key, true);
    }

    fn save(self) -> Result<(), Error> {
        let mut content = String::new();
        for (key, value) in self.map {
            let record = format!("{}\t{}\n", key, value);
            content.push_str(&record);
        }
        write("db.txt", content)
    }
}

fn main() {
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Please specify an item");

    let mut todo = Todo {
        map: HashMap::new(),
    };

    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("todo saved"),
            Err(why) => println!("An error occurred: {}", why),
        }
    }
}
