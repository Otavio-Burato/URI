// https://github.com/Otavio-Burato/URI/tree/main/rust/src
// 1005 - Média 1
// Rust (1.48) {beta}
use std::io;

/// # Entrada
/// O arquivo de entrada contém 2 valores com uma casa decimal cada um.
fn main() {
    let mut input_a = String::new();
    let mut input_b = String::new();

    io::stdin().read_line(&mut input_a).expect("Erro ao ler");
    io::stdin().read_line(&mut input_b).expect("Erro ao ler");

    println!("{}", saida(input_a, input_b));
}

/// # Saída
/// Imprima a mensagem "MEDIA" e a média do aluno conforme
/// exemplo abaixo, com 5 dígitos após o ponto decimal e
/// com um espaço em branco antes e depois da igualdade.
/// Utilize variáveis de dupla precisão (double) e como
/// todos os problemas, não esqueça de imprimir o fim de
/// linha após o resultado, caso contrário, você receberá "Presentation Error".
pub fn saida(input_a: String, input_b: String) -> String {
    let a: f64 = input_a.trim().parse().unwrap();
    let b: f64 = input_b.trim().parse().unwrap();

    format!("MEDIA = {:.5}", calcular_media(a, b))
}

fn calcular_media(a: f64, b: f64) -> f64 {
    ((a * 3.5) + (b * 7.5)) / (3.5 + 7.5)
}

#[test]
fn ex1005() {
    assert_eq!(
        saida(String::from("5.0"), String::from("7.1")),
        "MEDIA = 6.43182"
    );
    assert_eq!(
        saida(String::from("0.0"), String::from("7.1")),
        "MEDIA = 4.84091"
    );
    assert_eq!(
        saida(String::from("10.0"), String::from("10.0")),
        "MEDIA = 10.00000"
    );
}
