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

    #[test]
    fn ex1004() {
        assert_eq!(
            ex1004::saida(String::from("3"), String::from("9")),
            "PROD = 27"
        );
        assert_eq!(
            ex1004::saida(String::from("-30"), String::from("10")),
            "PROD = -300"
        );
        assert_eq!(
            ex1004::saida(String::from("0"), String::from("9")),
            "PROD = 0"
        );
    }

    #[test]
    fn ex1005() {
        assert_eq!(
            ex1005::saida(String::from("5.0"), String::from("7.1")),
            "MEDIA = 6.43182"
        );
        assert_eq!(
            ex1005::saida(String::from("0.0"), String::from("7.1")),
            "MEDIA = 4.84091"
        );
        assert_eq!(
            ex1005::saida(String::from("10.0"), String::from("10.0")),
            "MEDIA = 10.00000"
        );
    }

    #[test]
    fn ex1006() {
        assert_eq!(
            ex1006::saida(
                String::from("5.0"),
                String::from("6.0"),
                String::from("7.0")
            ),
            "MEDIA = 6.3"
        );
        assert_eq!(
            ex1006::saida(
                String::from("5.0"),
                String::from("10.0"),
                String::from("10.0")
            ),
            "MEDIA = 9.0"
        );
        assert_eq!(
            ex1006::saida(
                String::from("10.0"),
                String::from("10.0"),
                String::from("5.0")
            ),
            "MEDIA = 7.5"
        );
    }

    #[test]
    fn ex1007() {
        assert_eq!(
            ex1007::saida(
                String::from("5"),
                String::from("6"),
                String::from("7"),
                String::from("8")
            ),
            "DIFERENCA = -26"
        );
        assert_eq!(
            ex1007::saida(
                String::from("0"),
                String::from("0"),
                String::from("7"),
                String::from("8")
            ),
            "DIFERENCA = -56"
        );
        assert_eq!(
            ex1007::saida(
                String::from("5"),
                String::from("6"),
                String::from("-7"),
                String::from("8")
            ),
            "DIFERENCA = 86"
        );
    }

    #[test]
    fn ex1008() {
        assert_eq!(
            ex1008::saida(
                String::from("25"),
                String::from("100"),
                String::from("5.50")
            ),
            "NUMBER = 25\nSALARY = U$ 550.00"
        );
        assert_eq!(
            ex1008::saida(
                String::from("1"),
                String::from("200"),
                String::from("20.50")
            ),
            "NUMBER = 1\nSALARY = U$ 4100.00"
        );
        assert_eq!(
            ex1008::saida(
                String::from("6"),
                String::from("145"),
                String::from("15.55")
            ),
            "NUMBER = 6\nSALARY = U$ 2254.75"
        );
    }
}

mod ex1000;
mod ex1001;
mod ex1002;
mod ex1003;
mod ex1004;
mod ex1005;
mod ex1006;
mod ex1007;
mod ex1008;
