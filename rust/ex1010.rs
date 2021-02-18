// https://github.com/Otavio-Burato/URI/tree/main/rust/src
// 1010 - Cálculo Simples
// Rust (1.48) {beta}
use std::io;

/// # Entrada
/// O arquivo de entrada contém duas linhas de dados.
/// Em cada linha haverá 3 valores, respectivamente
/// dois inteiros e um valor com 2 casas decimais.
fn main() {
    let mut input_a = String::new();
    let mut input_b = String::new();

    io::stdin().read_line(&mut input_a).expect("Erro ao ler");
    io::stdin().read_line(&mut input_b).expect("Erro ao ler");

    println!("{}", saida(input_a, input_b));
}

/// # Saída
/// A saída deverá ser uma mensagem conforme o exemplo fornecido abaixo,
/// lembrando de deixar um espaço após os dois pontos e um espaço após o
/// "R$". O valor deverá ser apresentado com 2 casas após o ponto.
pub fn saida(input_a: String, input_b: String) -> String {
    let mut a = input_a.trim().split_whitespace();
    let mut b = input_b.trim().split_whitespace();

    a.next();
    b.next();

    format!("VALOR A PAGAR: R$ {:.2}", calc(a) + calc(b))
}

fn calc(mut a: std::str::SplitWhitespace) -> f64 {
    let number: f64;
    let price: f64;
    match a.next() {
        Some(x) => number = x.trim().parse().unwrap(),
        None => panic!("input invalido"),
    };
    match a.next() {
        Some(x) => price = x.trim().parse().unwrap(),
        None => panic!("input invalido"),
    };
    number * price
}

#[test]
fn ex1010() {
    assert_eq!(
        saida(String::from("12 1 5.30"), String::from("16 2 5.10")),
        "VALOR A PAGAR: R$ 15.50"
    );
    assert_eq!(
        saida(String::from("13 2 15.30"), String::from("161 4 5.20")),
        "VALOR A PAGAR: R$ 51.40"
    );
    assert_eq!(
        saida(String::from("1 1 15.10"), String::from("2 1 15.10")),
        "VALOR A PAGAR: R$ 30.20"
    );
}
