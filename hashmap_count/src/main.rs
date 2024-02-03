use std::collections::{hash_map, HashMap};

fn logic(numbers: Vec<i32>) -> Vec<(i32, i32)> {
let mut frequenies= HashMap::new();

    for n in numbers {
        let frequency = frequenies.entry(n).or_insert(0);
        *frequency += 1;
    }

    let mut result = Vec::new();

    for (num, frequency) in frequenies {
        result.push((num, frequency))
    }
    result
}

fn main() {
    let numbers = vec![1, 2, 3, 1, 1, 1, 4, 2, 4, 5, 543, 6543, 543];
    let result = logic(numbers);

    for (num, freq) in result {
    println!(
        "The frequency of {} is {}",
        num,
        freq);
    }
}
