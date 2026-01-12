//SEM PARAMÊTRO
/*use std::io;

fn main() {
    calcular_valor_estacionamento();
}

fn calcular_valor_estacionamento() -> f32 {
    let uma_hora: f32 = 8.0;

    println!("Infome quanto tempo seu veículo ficou estacionado (em horas): ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Falha ao ler a entrada");

    let horas: u32 = input.trim().parse().expect("Insira um número válido");

    let valor: f32;

    if horas == 0 {
        valor = 0.0;
    } else if horas == 1 {
        valor = uma_hora;
    } else {
        valor = uma_hora + (horas - 1) as f32 * 5.0;
    }

    println!("Valor do estacionamento foi: R$ {:.2}", valor);

    return valor;
}*/


//COM PARÂMETRO
use std::io;
fn main() {
    println!("Informe quantas horas ficou no estacionamento: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Falha ao ler a entrada");

    let horas:u32 = input.trim().parse().expect("Insira um número válido");
    let valor = calcular_valor_estacionamento(horas);

    println!("Valor do estacionamento foi: R$ {:.2}", valor);
}

fn calcular_valor_estacionamento(horas: u32) -> f32 {
    let uma_hora: f32 = 8.0;

    if horas == 0 {
        0.0
    } else if horas == 1 {
        uma_hora
    } else {
        uma_hora + (horas - 1) as f32 * 5.0
    }
}
