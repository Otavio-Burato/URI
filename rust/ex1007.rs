// https://github.com/Otavio-Burato/URI/tree/main/rust/src
// 1007 - Diferença
// Rust (1.48) {beta}
use std::io;

/// # Entrada
/// O arquivo de entrada contém 4 valores inteiros.
fn main() {
    let mut input_a = String::new();
    let mut input_b = String::new();
    let mut input_c = String::new();
    let mut input_d = String::new();

    io::stdin().read_line(&mut input_a).expect("Erro ao ler");
    io::stdin().read_line(&mut input_b).expect("Erro ao ler");
    io::stdin().read_line(&mut input_c).expect("Erro ao ler");
    io::stdin().read_line(&mut input_d).expect("Erro ao ler");

    println!("{}", saida(input_a, input_b, input_c, input_d));
}

/// # Saída
/// Imprima a mensagem DIFERENCA com todas as letras maiúsculas,
/// conforme exemplo abaixo, com um espaço em branco antes e depois da igualdade.
pub fn saida(input_a: String, input_b: String, input_c: String, input_d: String) -> String {
    let a: i32 = input_a.trim().parse().unwrap();
    let b: i32 = input_b.trim().parse().unwrap();
    let c: i32 = input_c.trim().parse().unwrap();
    let d: i32 = input_d.trim().parse().unwrap();

    format!("DIFERENCA = {}", diferenca(a, b, c, d))
}

fn diferenca(a: i32, b: i32, c: i32, d: i32) -> i32 {
    a * b - c * d
}

#[test]
fn ex1007() {
    assert_eq!(
        saida(
            String::from("5"),
            String::from("6"),
            String::from("7"),
            String::from("8")
        ),
        "DIFERENCA = -26"
    );
    assert_eq!(
        saida(
            String::from("0"),
            String::from("0"),
            String::from("7"),
            String::from("8")
        ),
        "DIFERENCA = -56"
    );
    assert_eq!(
        saida(
            String::from("5"),
            String::from("6"),
            String::from("-7"),
            String::from("8")
        ),
        "DIFERENCA = 86"
    );
}
