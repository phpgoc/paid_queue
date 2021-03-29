pub fn read_line_from_input() -> String {
    let mut str = String::new();
    std::io::stdin().read_line(&mut str).unwrap();
    str.trim_end().to_string()
}
