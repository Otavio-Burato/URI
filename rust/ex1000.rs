// https://github.com/Otavio-Burato/URI/tree/main/rust/src
// 1000 - Hello World!
// Rust (1.48) {beta}

/// # Entrada
/// Este problema não possui nenhuma entrada.
fn main() {
    println!("{}", saida());
}

/// # Saída
/// Você deve imprimir a mensagem "Hello World!" conforme o exemplo abaixo.
pub fn saida() -> String {
    format!("Hello World!")
}

#[test]
fn ex1000() {
    assert_eq!(saida(), "Hello World!");
}
