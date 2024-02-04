use std::collections::{hash_map, HashMap};

fn init_languges() -> HashMap<String, i32> {
    let mut languges = HashMap::new();
    languges.insert("Python".to_owned(), 1991);
    languges.insert("JavaScript".to_owned(), 1995);
    languges.insert("HTML/CSS".to_owned(), 1990);
    languges.insert("SQL".to_owned(), 1974);
    languges.insert("TypeScript".to_owned(), 2012);
    languges.insert("JavaScript".to_owned(), 1995);
    languges.insert("Bash/Shell".to_owned(), 1989);
    languges.insert("Java".to_owned(), 1995);
    languges.insert("C#".to_owned(), 2000);
    languges.insert("C++".to_owned(), 1985);
    languges.insert("C".to_owned(), 1972);
    languges.insert("PHP".to_owned(), 1995);
    languges.insert("PowerShell".to_owned(), 2006);
    languges.insert("Go".to_owned(), 2007);
    languges
}

fn main() {
    println!("Hello, world!");
}
