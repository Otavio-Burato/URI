// https://github.com/Otavio-Burato/URI/tree/main/rust/src
// 1009 - Salário com Bônus
// Rust (1.48) {beta}
use std::io;

/// # Entrada
/// O arquivo de entrada contém um texto (primeiro nome do vendedor)
/// e 2 valores de dupla precisão (double) com duas casas decimais,
/// representando o salário fixo do vendedor e montante total das
/// vendas efetuadas por este vendedor, respectivamente.
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
/// Imprima o total que o funcionário deverá receber, conforme exemplo fornecido.
pub fn saida(_input_a: String, input_b: String, input_c: String) -> String {
    let b: f64 = input_b.trim().parse().unwrap();
    let c: f64 = input_c.trim().parse().unwrap();

    format!("TOTAL = R$ {:.2}", salary(b, c))
}

fn salary(b: f64, c: f64) -> f64 {
    b + (0.15 * c)
}
