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
    languges.insert("Rust".to_owned(), 2010);
    languges
}

fn calculate_heights(years_active: &mut HashMap<String, i32>) -> HashMap<String, i32> {
    for year in years_active.values_mut() {
    *year = (*year - 2024).abs();
    }

    let min_year = *years_active.values().min().unwrap_or(&0);
    let max_year = *years_active.values().max().unwrap_or(&0);

    let mut weights: HashMap<String, i32> = HashMap::new();

    for (lang, &year) in years_active.iter() {
        let normalized_year = (year - min_year) as f64 / (max_year - min_year) as f64;
        let weight = (normalized_year * 99.0) as i32 + 1;
        weights.insert(lang.to_owned(), weight);
    }
    weights
}

fn main() {
    let mut languages = init_languges();
    let weights = calculate_heights(&mut languages);

    println!("Lnaguage weighing from 1-100 by age(1 is newest and 100 is oldest:");
    for (lang, weight) in &weights {
    println!("{}: {}", lang, weight);
    }
}
