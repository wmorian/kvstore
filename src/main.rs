use std::collections::HashMap;

fn main() {
    let mut args = std::env::args().skip(1);
    let key = args.next().unwrap();
    let value = args.next().unwrap();

    // let contents = format!("{}\t{}\n", key, value);
    // let result = std::fs::write("kv.db", contents);

    let db = Database::new().expect("Creating database failed!");
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let mut map = HashMap::new();

        let mut path = std::path::PathBuf::new();
        path.push("kv.db");
        if !path.exists() {
            std::fs::File::create(path)?;
        }

        let content = std::fs::read_to_string("kv.db")?;
        for line in content.lines() {
            let mut chunks = line.splitn(1, '\t');
            let key = chunks.next().expect("No key!");
            let value = chunks.next().expect("No value!");
            map.insert(key.to_owned(), value.to_owned());
        }

        Ok(Database {
            map: map,
        })
    }
}