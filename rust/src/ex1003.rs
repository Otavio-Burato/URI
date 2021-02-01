// https://github.com/Otavio-Burato/URI/tree/main/rust/src
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
/// Imprima a mensagem "SOMA" com todas as letras maiúsculas,
/// com um espaço em branco antes e depois da igualdade seguido
/// pelo valor correspondente à soma de A e B. Como todos os
/// problemas, não esqueça de imprimir o fim de linha após o
/// resultado, caso contrário, você receberá "Presentation Error".
pub fn saida(input_a: String, input_b: String) -> String {
    let a: i32 = input_a.trim().parse().unwrap();
    let b: i32 = input_b.trim().parse().unwrap();

    format!("SOMA = {}", a + b)
}
