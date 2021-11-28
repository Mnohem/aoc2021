pub fn easy_input(input_file: &str) -> String {
    std::fs::read_to_string(format!("../input/{}", input_file))
        .expect("Could not read file").trim().to_owned()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = crate::easy_input("cum.txt");
        assert_eq!(result, "fart and bitches");
    }
}
