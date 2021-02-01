mod ex1000;
mod ex1001;
mod ex1002;
mod ex1003;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1000() {
        assert_eq!(ex1000::saida(), "Hello World!");
    }

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

    #[test]
    fn ex1003() {
        assert_eq!(
            ex1003::saida(String::from("30"), String::from("10")),
            "SOMA = 40"
        );
        assert_eq!(
            ex1003::saida(String::from("-30"), String::from("10")),
            "SOMA = -20"
        );
        assert_eq!(
            ex1003::saida(String::from("0"), String::from("0")),
            "SOMA = 0"
        );
    }
}
