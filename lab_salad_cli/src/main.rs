use lab_salad_cli::{pick_one, read_line};

fn main() {
    let mut fruits: Vec<String> = Vec::new();
    let mut option = String::new();

    while option != "0" {
        option.clear();
        println!("-----------------------------\n(1) Adicioanr frutas.\n(2) Escolher uma fruta aleatoriamente.\n(3) Adicioanr frutas aleatorias.\n(0) para fechar o programa.");
        option = read_line();
        match option.as_str() {
            "0" => break,
            "1" => {
                loop {
                    println!("Insira a fruta que deseja adicionar (caso deseja parar digite \"sair\")");
                    let fruta = read_line();
                    if fruta == "sair".to_lowercase() {
                        break;
                    }
                    fruits.push(fruta);
                }
            }
            "2" => pick_one(&fruits),
            default => println!("Numero errado.")
        }
    }
}
