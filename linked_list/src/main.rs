use std::collections::{LinkedList, VecDeque};
use rand::prelude::SliceRandom;
use rand::thread_rng;

pub fn simple() {
    let mut fruit: VecDeque<&str> = VecDeque::new();
    fruit.push_back("Arbutus");
    fruit.push_front("Loquat");
    fruit.push_back("Straberry Tree Berry");

    // Scramble
    let mut rng = thread_rng();
    let mut fruit: Vec<_> = fruit.into_iter().collect();
    fruit.shuffle(&mut rng);

    let mut fruit: LinkedList<_> = fruit.into_iter().collect();

    print!("Fruit Salad: [");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}]", item);
        }
    }
}

fn main() {
    simple();
}