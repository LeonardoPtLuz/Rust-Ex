use std::io;

fn main() {
    let mut maior: i32;
    let mut menor: i32;

    println!("Digite o 1º número: ");
    let mut numero = String::new();
    io::stdin().read_line(&mut numero).expect("Falha ao ler a entreada");
    let numero: i32 = numero.trim().parse().expect("Nº inválido");
    maior = numero;
    menor = numero;

    for i in 2..=5 {
        println!("Digite o {}º número: ", i);
        let mut numero = String::new();
        io::stdin().read_line(&mut numero).expect("Falha ao ler a entreada");
        let numero: i32 = numero.trim().parse().expect("Nº inválido");

        if numero > maior {
            maior = numero;
        } else if numero < menor {
            menor = numero;
        }
    }

    print!("O maior número é: {} \nO menor número é: {}", maior, menor);
}