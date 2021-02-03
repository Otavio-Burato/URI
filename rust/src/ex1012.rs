// https://github.com/Otavio-Burato/URI/tree/main/rust/src
// 1012 - Área
// Rust (1.48) {beta}
use std::io;

/// # Entrada
/// O arquivo de entrada contém três valores com um dígito após o ponto decimal.
fn main() {
    let mut input_a = String::new();

    io::stdin().read_line(&mut input_a).expect("Erro ao ler");

    println!("{}", saida(input_a));
}

/// # Saída
/// O arquivo de saída deverá conter 5 linhas de dados.
/// Cada linha corresponde a uma das áreas descritas acima,
/// sempre com mensagem correspondente e um espaço entre os
/// dois pontos e o valor. O valor calculado deve ser
/// apresentado com 3 dígitos após o ponto decimal.
pub fn saida(input_a: String) -> String {
    let mut a = input_a.trim().split_whitespace();

    format!("{}", calc(a))
}

fn calc(mut input: std::str::SplitWhitespace) -> String {
    let a: f64;
    let b: f64;
    let c: f64;
    const PI: f64 = 3.14159;
    match input.next() {
        Some(x) => a = x.parse().unwrap(),
        None => panic!("input invalido"),
    };
    match input.next() {
        Some(x) => b = x.parse().unwrap(),
        None => panic!("input invalido"),
    };
    match input.next() {
        Some(x) => c = x.parse().unwrap(),
        None => panic!("input invalido"),
    };
    format!(
        "TRIANGULO: {:.3}\nCIRCULO: {:.3}\nTRAPEZIO: {:.3}\nQUADRADO: {:.3}\nRETANGULO: {:.3}",
        ((a * c) / 2.0),
        (PI * c.powi(2)),
        ((a + b) / 2.0) * c,
        (b * b),
        (a * b)
    )
}
