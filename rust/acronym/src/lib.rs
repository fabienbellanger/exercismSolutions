pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| c.is_whitespace() || c == '-')
        .map(|w| {
            let words: String = w.chars().filter(|c| c.is_alphabetic()).collect();

            let all_uppercase = words.chars().all(|c| c.is_ascii_uppercase());
            if all_uppercase && !words.is_empty() {
                return words.chars().next().unwrap().to_string();
            }

            let mut s = String::new();
            for (i, c) in words.chars().enumerate() {
                if i == 0 || c.is_ascii_uppercase() {
                    s.push(c);
                }
            }

            s
        })
        .collect::<String>()
        .to_uppercase()
}
