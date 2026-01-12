use std::io;

fn fatorial(numero: u64) -> u64 {
    if numero <= 1 {
        return 1;
    } else {
       return numero * fatorial(numero - 1);
    }
}

fn main() {
    println!("Digite um número inteiro para calcular o fatorial: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erro ao ler!");

    let numero: u64 = input.trim().parse().expect("Número inválido!");

    println!("{}! = {}", numero, fatorial(numero));
}