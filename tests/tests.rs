#[cfg(test)]
mod tests {
    use iftcl_translator::{translate, Language};

    #[test]
    fn some_2words() {
        let var = String::from("Qw");
        let result = translate(var, Language::English, Language::Ukrainian);
        assert_eq!(result, "йц")
    }

    #[test]
    fn qwerty_check() {
        let var = String::from("Qwerty");
        let result = translate(var, Language::English, Language::Ukrainian);
        assert_eq!(result, "йцукен")
    }

    #[test]
    fn all_is_numbers() {
        let var = String::from("1234");
        let result = translate(var, Language::English, Language::Ukrainian);
        assert_eq!(result, "1234")
    }

    #[test]
    fn only_know_letters() {
        let var = String::from("Hb,f yf zgjycmrsq - フィッシュ");
        let result = translate(var, Language::English, Language::Ukrainian);
        assert_eq!(result, "риба на японській - フィッシュ")
    }
}
