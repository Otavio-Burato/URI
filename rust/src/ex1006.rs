// https://github.com/Otavio-Burato/URI/tree/main/rust/src
// 1006 - Média 2
// Rust (1.48) {beta}
use std::io;

/// # Entrada
/// O arquivo de entrada contém 3 valores com uma casa decimal, de dupla precisão (double).
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
/// Imprima a mensagem "MEDIA" e a média do aluno conforme
/// exemplo abaixo, com 1 dígito após o ponto decimal e com
/// um espaço em branco antes e depois da igualdade. Assim
/// como todos os problemas, não esqueça de imprimir o fim
/// de linha após o resultado, caso contrário, você receberá "Presentation Error".
pub fn saida(input_a: String, input_b: String, input_c: String) -> String {
    let a: f64 = input_a.trim().parse().unwrap();
    let b: f64 = input_b.trim().parse().unwrap();
    let c: f64 = input_c.trim().parse().unwrap();

    format!("MEDIA = {:.1}", calcular_media(a, b, c))
}

fn calcular_media(a: f64, b: f64, c: f64) -> f64 {
    ((a * 2.0) + (b * 3.0) + (c * 5.0)) / (2.0 + 3.0 + 5.0)
}
