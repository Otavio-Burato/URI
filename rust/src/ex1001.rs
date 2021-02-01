// https://github.com/Otavio-Burato/URI/tree/main/rust/src
use std::io;

/// # Entrada
/// A entrada contém 2 valores inteiros.
fn main() {
    let mut input_a = String::new();
    let mut input_b = String::new();

    io::stdin().read_line(&mut input_a).expect("Erro ao ler");
    io::stdin().read_line(&mut input_b).expect("Erro ao ler");

    println!("{}", saida(input_a, input_b));
}

/// # Saída
/// Imprima a mensagem `"X = " (letra X maiúscula)`
/// seguido pelo valor da variável X e pelo final de linha.
/// Cuide para que tenha um espaço antes e depois do sinal de igualdade.
pub fn saida(input_a: String, input_b: String) -> String {
    let a: i32 = input_a.trim().parse().unwrap();
    let b: i32 = input_b.trim().parse().unwrap();

    format!("X = {}", a + b)
}
