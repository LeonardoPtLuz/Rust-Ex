// fn main() {
//     println!("Hello, world!");
// }


/*Crie um programa que declare uma variável imutável para o seu nome (como string), 
  uma mutável para a sua idade (inteiro), incremente a idade em 1 e 
  imprima uma mensagem como "Olá, [nome]! Você tem [idade] anos agora."
*/
// fn main() {
//     let nome: &str = "Leonardo";
//     let mut idade: i32 = 29;

//     idade += 1;

//     println!("Olá, meu nome é {} e eu tenho {} anos.", nome, idade);
// }

//-------------------------------------------------------------------------------------------------------------------------------

/*Tarefa: Escreva uma função que recebe um número inteiro e retorna se ele é par ou ímpar (como uma string). 
  No main, use um loop for para testar a função de 1 a 10 e imprimir os resultados.
 */

// fn main () {
//     for i in 1..=10 {
//         let resultado = par_ou_impar(i);
//         println!("O número {} é {}", i, resultado)
//     }
// }

// fn par_ou_impar(num: i32) -> String {
//     if num % 2 == 0 {
//         "par".to_string()
//     } else {
//         "ímpar".to_string()
//     }
// }

//-------------------------------------------------------------------------------------------------------------------------------

/*Tarefa: Crie uma função que recebe uma string por borrowing e a imprime em maiúsculas sem modificar a original. 
  No main, crie uma String, chame a função e imprima a original para provar que não mudou.
 */

//  fn main() {
//     let mensagem = String::from("oi vc!");
//     imprime_maiusculas(&mensagem);
//     print!("Original: {}", mensagem);  
//  }

//  fn imprime_maiusculas(texto: &String) {
//     println!("Maiúsculas: {}", texto.to_ascii_uppercase());
//  }

 //-------------------------------------------------------------------------------------------------------------------------------

 /*Tarefa: Defina um enum Forma com variantes Circulo(raio: f64) e Retangulo(largura: f64, altura: f64). 
   Crie uma struct Objeto com um campo forma: Forma. Escreva um método para calcular a área e teste no main.
 */

//  enum Forma {
//     Circulo(f64),
//     Retangulo(f64, f64),
//  }

//  struct Objeto {
//     forma: Forma,
//  }

//  impl Objeto {
//      fn area(self) -> f64 {
//         match &self.forma {
//             Forma::Circulo(raio) => std::f64::consts::PI * raio * raio,
//             Forma::Retangulo(largura, altura ) => *largura * altura,
//         }
//      }
//  }

//  fn main () {
//     let circulo = Objeto { forma: Forma::Circulo(5.0) };
//     let retangulo = Objeto { forma: Forma::Retangulo(4.0, 6.0) };

//     println!("Área do círculo: {}", circulo.area());
//     println!("Área do retângulo: {}", retangulo.area());
//  }

//-------------------------------------------------------------------------------------------------------------------------------

 /*Tarefa: Crie um vetor de inteiros, adicione números de 1 a 5,
   então use um loop para dobrar cada valor (mutando o vetor) e imprima o resultado.
 */

fn main() {
    let mut numeros: Vec<i32> = Vec::new();
    
    for i in 1..=5 {
        numeros.push(i); // Adiciona ao vetor
    }

    // Dobrando os valores com borrowing mutável
    for num in &mut numeros {
        *num *= 2;
    }

    println!("Vetor dobrado: {:?}", numeros); // {:?} para debug print
}

//-------------------------------------------------------------------------------------------------------------------------------