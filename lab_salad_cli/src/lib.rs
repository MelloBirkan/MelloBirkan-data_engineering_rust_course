use rand::seq::SliceRandom; // Importa o trait necessário para usar o método shuffle
use rand::thread_rng; // Importa a função thread_rng

pub fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input) // Lê a linha e armazena na string 'input'
        .expect("Falha ao ler a linha"); // Trata erros potenciais
    input.trim().to_owned()
}

pub fn pick_one(v: &[String]) {
    let mut rng = thread_rng();
    let chosen_fruit = v.choose(&mut rng);

    match chosen_fruit {
        Some(fruit) => println!("A fruta escolhida foi: {}", fruit), // Converte &String para String
        None =>println!("Não foi possível retornar uma fruta"),
    }
}