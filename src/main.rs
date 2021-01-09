use std::collections::HashMap;

fn main() -> std::io::Result<()> {
    let mut args = std::env::args().skip(1);
    let key = args.next().unwrap();
    let value = args.next().unwrap();

    let mut db = Database::new().expect("Creating database failed!");
    db.insert(key.clone(), value.clone());
    db.insert(key.to_uppercase(), value);
    db.flush()
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let mut map = HashMap::new();

        // let mut path = std::path::PathBuf::new();
        // path.push("kv.db");
        // if !path.exists() {
        //     std::fs::File::create(path)?;
        // }

        let content = std::fs::read_to_string("kv.db")?;
        print!("{}", content);
        for line in content.lines() {
            let mut chunks = line.splitn(2, '\t');
            let key = chunks.next().expect("No key!");
            let value = chunks.next().expect("No value!");
            map.insert(key.to_owned(), value.to_owned());
        }

        Ok(Database {
            map,
        })
    }

    // this is a method, it is actually just a syntactic sugar for when you call ist
    // so instead of 
    //      let db = Database::new();
    //      Database::insert(database, key, value);
    // you can do
    //      let db = Database::new();
    //      db.insert(key, value);
    fn insert(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    fn flush(self) -> std::io::Result<()> { // is the same as Result<(), std::io::Error>
        let mut contents = String::new();
        for (key, value) in &self.map {
            contents.push_str(key);
            contents.push('\t');
            contents.push_str(value);
            contents.push('\n');
        }

        std::fs::write("kv.db", contents)
    }
}

impl Drop for Database {

    fn drop(&mut self) {
                let mut contents = String::new();
        for (key, value) in &self.map {
            contents.push_str(key);
            contents.push('\t');
            contents.push_str(value);
            contents.push('\n');
        }

        let _ = std::fs::write("kv.db", contents);
    }
}