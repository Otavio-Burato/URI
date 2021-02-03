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
    let a = input_a.trim().parse().unwrap();

    format!("VOLUME = {:.3}", calc(a))
}

fn calc(mut r: f64) -> f64 {
    const PI: f64 = 3.14159;
    (4.0 / 3.0) * PI * r.powi(3)
}
