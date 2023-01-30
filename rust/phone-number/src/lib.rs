pub fn number(user_number: &str) -> Option<String> {
    let mut s = user_number
        .trim()
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<Vec<char>>();

    if s.len() == 11 && s[0] == '1' {
        s.remove(0);
    }

    if s.len() != 10 {
        return None;
    }

    if s[0] == '0' || s[0] == '1' || s[3] == '0' || s[3] == '1' {
        return None;
    }
    Some(s.into_iter().collect())
}
