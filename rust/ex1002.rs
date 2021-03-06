// https://github.com/Otavio-Burato/URI/tree/main/rust/src
// 1002 - Área do Círculo
// Rust (1.48) {beta}
use std::io;

/// # Entrada
/// A entrada contém um valor de ponto flutuante (dupla precisão),
/// no caso, a variável raio.
fn main() {
    let mut input_a = String::new();

    io::stdin().read_line(&mut input_a).expect("Erro ao ler");

    println!("{}", saida(input_a));
}

/// # Saída
/// Apresentar a mensagem "A=" seguido pelo valor da variável area,
/// conforme exemplo abaixo, com 4 casas após o ponto decimal.
/// Utilize variáveis de dupla precisão (double).
/// Como todos os problemas, não esqueça de imprimir o fim de linha
/// após o resultado, caso contrário, você receberá "Presentation Error".
pub fn saida(input_a: String) -> String {
    const PI: f64 = 3.14159;
    let a: f64 = input_a.trim().parse().unwrap();

    format!("A={:.4}", PI * (a * a))
}

#[test]
fn ex1002() {
    assert_eq!(saida(String::from("2.00")), "A=12.5664");
    assert_eq!(saida(String::from("100.64")), "A=31819.3103");
    assert_eq!(saida(String::from("150.00")), "A=70685.7750");
}
