mod ex1001;
mod ex1002;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1001() {
        assert_eq!(
            ex1001::saida(String::from("10"), String::from("9")),
            "X = 19"
        );
        assert_eq!(
            ex1001::saida(String::from("-10"), String::from("4")),
            "X = -6"
        );
        assert_eq!(
            ex1001::saida(String::from("15"), String::from("-7")),
            "X = 8"
        );
    }

    #[test]
    fn ex1002() {
        assert_eq!(ex1002::saida(String::from("2.00")), "A=12.5664");
        assert_eq!(ex1002::saida(String::from("100.64")), "A=31819.3103");
        assert_eq!(ex1002::saida(String::from("150.00")), "A=70685.7750");
    }
}
