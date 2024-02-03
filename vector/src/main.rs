use rand::seq::SliceRandom;
use rand::thread_rng; // rand is a random number generation lib in Rust
use std::collections::VecDeque; // Deque

pub fn simple_vector() {
    let mut fruit = vec!["Orange", "Fig", "Pomegranate", "Cherry", "Apple"];

    let mut rng = thread_rng();

    print!("Fruit Salad: [");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}]", item);
        }
    }
}

pub fn deque() {
    let mut fruit: VecDeque<&str> = VecDeque::new();
    fruit.push_back("Arbutus");
    fruit.push_front("Loquat");
    fruit.push_back("Straberry Tree Berry");

    // Scramble
    let mut rng = thread_rng();
    // into_iter toma posse, iter apenas a referencia (&)
    let mut fruit: Vec<_> = fruit.into_iter().collect();
    fruit.shuffle(&mut rng);

    fruit.iter().for_each(|fruit| print!("{}, ", fruit));
}
fn main() {
    deque();
}