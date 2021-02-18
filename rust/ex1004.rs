// https://github.com/Otavio-Burato/URI/tree/main/rust/src
// 1004 - Produto Simples
// Rust (1.48) {beta}
use std::io;

/// # Entrada
/// O arquivo de entrada contém 2 valores inteiros.
fn main() {
    let mut input_a = String::new();
    let mut input_b = String::new();

    io::stdin().read_line(&mut input_a).expect("Erro ao ler");
    io::stdin().read_line(&mut input_b).expect("Erro ao ler");

    println!("{}", saida(input_a, input_b));
}

/// # Saída
/// Imprima a mensagem "PROD" e a variável PROD
/// conforme exemplo abaixo, com um espaço em branco
/// antes e depois da igualdade. Não esqueça de imprimir
/// o fim de linha após o produto, caso contrário seu programa
/// apresentará a mensagem: “Presentation Error”.
pub fn saida(input_a: String, input_b: String) -> String {
    let a: i32 = input_a.trim().parse().unwrap();
    let b: i32 = input_b.trim().parse().unwrap();

    format!("PROD = {}", a * b)
}

#[test]
fn ex1004() {
    assert_eq!(
        saida(String::from("3"), String::from("9")),
        "PROD = 27"
    );
    assert_eq!(
        saida(String::from("-30"), String::from("10")),
        "PROD = -300"
    );
    assert_eq!(
        saida(String::from("0"), String::from("9")),
        "PROD = 0"
    );
}
