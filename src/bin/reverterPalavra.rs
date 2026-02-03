use std::io;

fn main() {
    println!("Digite a palavra que você deseja reverter: ");
    let mut input_palavra = String::new();
    io::stdin().read_line(&mut input_palavra).expect("Erro ao ler a entrada!");

    let palavra = input_palavra.trim();
    let palavra_revertida = reverter_palavra(palavra);
    println!("Palavra revertida: {}", palavra_revertida);
}

fn reverter_palavra(palavra: &str) -> String {
    palavra.chars().rev().collect()
}