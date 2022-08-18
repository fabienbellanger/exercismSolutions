pub fn encode(source: &str) -> String {
    let mut encoded_str = String::new();
    let mut chars = source.chars().peekable();
    let mut counter = 1;

    while let Some(current_char) = chars.next() {
        if Some(&current_char) == chars.peek() {
            counter += 1;
        } else {
            if counter > 1 {
                encoded_str.push_str(&counter.to_string());
            }
            encoded_str.push(current_char);

            counter = 1;
        }
    }
    
    encoded_str
}

pub fn decode(source: &str) -> String {
    let mut s = String::new();
    let mut number = String::new();

    for c in source.chars() {
        if c.is_ascii_digit() {
            number.push(c);
        } else {
            let n = number.parse::<usize>().unwrap_or(1);
            let part = (0..n).map(|_| c).collect::<String>();
            s.push_str(&part);

            number = "".to_owned();
        }
    }

    s
}

fn _encode_old(source: &str) -> String {
    let mut s = String::new();
    let mut current_char = None;
    let mut counter = 0;
    let source_length = source.len();

    for (idx, c) in source.chars().enumerate() {
        if current_char.is_none() {
            current_char = Some(c);
            counter += 1;
        } else if Some(c) == current_char {
            counter += 1;
        } else {
            if counter > 1 {
                s.push_str(&counter.to_string());
            }
            s.push(current_char.unwrap_or_default());
            current_char = Some(c);
            counter = 1;
        }

        if idx == source_length - 1 {
            if counter > 1 {
                s.push_str(&counter.to_string());
            }
            s.push(c);
        }
    }

    s
}
