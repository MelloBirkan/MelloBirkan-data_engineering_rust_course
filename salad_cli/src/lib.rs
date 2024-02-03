use rand::prelude::SliceRandom;
use rand::thread_rng;

pub fn create_fruit_salad(num_fruits: usize) -> Vec<String> {
    let fruits = vec![
        "Arbutus".to_owned(),
        "Strawberry".to_owned(),
        "Cherry".to_owned(),
        "Pear".to_owned(),
        "Apple".to_owned()
    ];

    let mut rng = thread_rng();
    let mut fruits = fruits;
    fruits.shuffle(&mut rng);

    fruits.into_iter().take(num_fruits).collect()


}