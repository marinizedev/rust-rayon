// rustrayon/src/main.rs

use rayon::prelude::*;

fn main() {
    // Cria um vetor com números 1 até 10.000.000
    let numbers: Vec<i32> = (1..=10_000_000).collect();

    // Usa Rayon para calcular a soma dos quadrados em paralelo
    let sum_of_squares: u128 = numbers
        .par_iter()                   
        .map(|&x| (x as u128) * (x as u128))
        .sum();
    println!("Soma dos quadrados: {}", sum_of_squares);

    // Filtra os números pares em paralelo e coleta em um vetor
    let even_numbers: Vec<i32> = numbers
        .par_iter()
        .filter(|&&x| x % 2 == 0)
        .cloned()     
        .collect();
    println!("Quantidade de números pares: {}", even_numbers.len());
}
