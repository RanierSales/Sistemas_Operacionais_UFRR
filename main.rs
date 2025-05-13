
use std::thread::{self, Scope};
use std::io;

fn main() {
    println!("Digite 3 números separados por espaço:");

    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).unwrap();

    let numeros: Vec<u64> = entrada
        .trim()
        .split_whitespace() // divide por espaços
        .filter_map(|s| s.parse().ok()) // tenta converter para u64
        .collect();

    thread::scope( |scope: &Scope| {

        scope.spawn(|| {
            let mut resultado = 1;
            for i in 1..=numeros[0] {
                resultado *= i;
            }
            println!("O fatorial de {} e {}", numeros[0], resultado);
        });

        scope.spawn(|| {
            let mut resultado = 1;
            for i in 1..=numeros[1] {
                resultado *= i;
            }
            println!("O fatorial de {} e {}", numeros[1], resultado);
        });

        scope.spawn(|| {
            let mut resultado = 1;
            for i in 1..=numeros[2] {
                resultado *= i;
            }
            println!("O fatorial de {} e {}", numeros[2], resultado);
        });
    })
}
