mod args;

use args::{EntityType, TodustArgs};
use clap::Parser;

fn main() {
    let args = TodustArgs::parse();

    match args.command {
        EntityType::Create(task) => println!("TESTE, {:?}", task.title),
        _ => println!("Nao deu certo"),
    }
}
