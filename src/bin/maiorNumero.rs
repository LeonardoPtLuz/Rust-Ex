use std::io;

fn main () {
    let mut input = String::new();

    println!("Digite um número: ");
    io::stdin().read_line(&mut input).expect("Falha ao ler a linha");

    let mut maior: i32 = input.trim().parse().expect("Digite apenas números inteiros!");
    input.clear();

    for _ in 1..5 {
        println!("Digite um número: ");
        io::stdin().read_line(&mut input).expect("Falha ao ler a linha");
        let numero: i32 = input.trim().parse().expect("Digite apenas números inteiros!");
        input.clear();

        if numero > maior {
            maior = numero;
        }
    }

    println!("O maior número é: {}", maior);
}