pub fn disemvowel(s: &str) -> String {
    return s.clone().to_string().replace(&['a', 'A','e', 'E', 'i', 'I', 'o', 'O', 'u', 'U'], "");
}