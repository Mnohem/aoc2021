pub fn easy_input(input_name: &str) -> String {
    //input file should be the same name as crate for aoc
    //so crate "day1" should have input "day1
    std::fs::read_to_string(format!("../input/{}", input_name))
        .expect("Could not read file").trim().to_owned()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = crate::easy_input("test.txt");
        assert_eq!(result, "foo and bar");
    }
}
