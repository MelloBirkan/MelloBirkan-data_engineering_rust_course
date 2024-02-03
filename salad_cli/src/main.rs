use salad_cli::create_fruit_salad;
use clap::Parser;

#[derive(Parser)]
#[clap(
version = "1.0",
author = "Marcello <mgbirkan@icloud.com>",
about= "Number of fruits to include in the salad"
)]
struct Opts {
    #[clap(short, long)]
    number: usize
}

fn main() {
    let opts: Opts = Opts::parse();
    let num_fruits: usize = opts.number;
    create_fruit_salad(num_fruits);

    println!("Created Fruit salad with {} fruits: {:?}",
    num_fruits,
    create_fruit_salad(num_fruits)
    );
}