use std::{collections::HashMap, io::{Error, Read}, fs::{write, OpenOptions}, str::FromStr};

struct Todo {
    // use rust built in HashMap to store key - val pairs
    map: HashMap<String, bool>,
}

impl Todo {
    fn new() -> Result<Todo, Error> {
        let mut openOptions = OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open("db.txt");

        let mut content = String::new();

        openOptions.unwrap().read_to_string(&mut content)?;

        // let map: HashMap<String, bool> = content
        // .lines()
        // // get each line and converts to an vector of strings [["pegar o lixo", "true"]...]
        // .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
        // // transform to tuples (key, value) => [("pegar o lixo", "true")...]
        // .map(|v| (v[0], v[1]))
        // // get each tuple's values and convert it back to a HashMap of <String, Bool> with the given values (k, v)
        // .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap()))
        // .collect();

        // Ok(Todo {map})

        // OR SIMPLER

        // allocate an empty HashMap
        let mut map = HashMap::new();

        // loop over each lines of the file
        for entries in content.lines() {
            // split and bind values
            let mut values = entries.split('\t');
            let key = values.next().expect("No Key");
            let value = values.next().expect("No Value");

            // insert them into HashMap
            map.insert(String::from(key), bool::from_str(value).unwrap());
        }

        // Return Ok
        Ok(Todo {map})
    }

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

    let mut todo = Todo::new().expect("Initialisation of db failed");

    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("todo saved"),
            Err(why) => println!("An error occurred: {}", why),
        }
    }
}
