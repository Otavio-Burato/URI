// https://github.com/Otavio-Burato/URI/tree/main/rust/src
// 1008 - Salário
// Rust (1.48) {beta}
use std::io;

/// # Entrada
/// O arquivo de entrada contém 2 números inteiros e 1 número
/// com duas casas decimais, representando o número, quantidade
/// de horas trabalhadas e o valor que o funcionário recebe por
/// hora trabalhada, respectivamente.
fn main() {
    let mut input_a = String::new();
    let mut input_b = String::new();
    let mut input_c = String::new();

    io::stdin().read_line(&mut input_a).expect("Erro ao ler");
    io::stdin().read_line(&mut input_b).expect("Erro ao ler");
    io::stdin().read_line(&mut input_c).expect("Erro ao ler");

    println!("{}", saida(input_a, input_b, input_c));
}

/// # Saída
/// Imprima o número e o salário do funcionário, conforme
/// exemplo fornecido, com um espaço em branco antes e depois
/// da igualdade. No caso do salário, também deve haver um espaço
/// em branco após o $.
pub fn saida(input_a: String, input_b: String, input_c: String) -> String {
    let a: i32 = input_a.trim().parse().unwrap();
    let b: i32 = input_b.trim().parse().unwrap();
    let c: f64 = input_c.trim().parse().unwrap();

    format!("NUMBER = {}\nSALARY = U$ {:.2}", a, salary(b, c))
}

fn salary(b: i32, c: f64) -> f64 {
    b as f64 * c
}

#[test]
fn ex1008() {
    assert_eq!(
        saida(
            String::from("25"),
            String::from("100"),
            String::from("5.50")
        ),
        "NUMBER = 25\nSALARY = U$ 550.00"
    );
    assert_eq!(
        saida(
            String::from("1"),
            String::from("200"),
            String::from("20.50")
        ),
        "NUMBER = 1\nSALARY = U$ 4100.00"
    );
    assert_eq!(
        saida(
            String::from("6"),
            String::from("145"),
            String::from("15.55")
        ),
        "NUMBER = 6\nSALARY = U$ 2254.75"
    );
}
