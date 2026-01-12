use std::io::{self, Write};

fn main() {

    print!("Digite o seu nome: ");
    io::stdout().flush().unwrap();

    let mut nome: String = String::new();
    io::stdin().read_line(&mut nome).expect("Falha ao ler a entrada!");

    let nome = nome.trim();

    println!("Oi, {}!", nome);
}