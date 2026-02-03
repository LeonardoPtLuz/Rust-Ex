use std::io;

fn main() {
    println!("Quantos números terá o vetor?: ");
    let mut input_quantidade_vetor = String::new();
    io::stdin().read_line(&mut input_quantidade_vetor).expect("Falha ao ler a linha!");
    
    let quantidade_vetor: usize = input_quantidade_vetor.trim().parse().expect("Insira apenas números inteiros!");

    let mut vetor_numeros: Vec<i32> = Vec::new();

    for i in 0..quantidade_vetor {
        println!("insira o {}º número de {}:", i + 1, quantidade_vetor);
        let mut input_numero = String::new();
        io::stdin().read_line(&mut input_numero).expect("Falha ao ler a linha!");

        let numero: i32 = input_numero.trim().parse().expect("Insira apenas números inteiros!");
        vetor_numeros.push(numero);
    }

    let maior_numero = maior_numero_vetor(&vetor_numeros);
    
    print!("Todos os números do vetror são: {}", vetor_numeros.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(", "));
    print!("\nO maior número do vetor é: {}", maior_numero)
}

fn maior_numero_vetor(vetor: &[i32]) -> i32 {
    let mut maior = vetor[0];
    for &numero in vetor.iter() {
        if numero > maior {
            maior = numero;
        }
    }
    return maior;
}