pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    let brackets: Vec<char> = string
        .chars()
        .filter(|c| ['(', '[', '{', ')', ']', '}'].contains(c))
        .collect();

    let mut valid = true;
    for bracket in &brackets {
        if ['(', '[', '{'].contains(bracket) {
            stack.push(*bracket);
        } else {
            let last = stack.pop();
            valid = match last {
                Some(c) => match c {
                    '(' => ')' == *bracket,
                    '{' => '}' == *bracket,
                    _ => ']' == *bracket,
                },
                None => false,
            };

            if !valid {
                break;
            }
        }
    }

    valid && stack.is_empty()
}
