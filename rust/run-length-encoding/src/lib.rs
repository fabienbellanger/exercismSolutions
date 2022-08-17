use std::fmt::Write;

pub fn encode(source: &str) -> String {
    let mut s = String::new();

    if source.is_empty() {
        return s;
    }

    let mut current_char = None;
    let mut counter = 0;
    for (idx, c) in source.chars().enumerate() {
        if current_char.is_none() {
            current_char = Some(c);
            counter += 1;
        } else if Some(c) == current_char {
            counter += 1;

            if idx == source.len() - 1 {
                if counter == 1 {
                    s.push(current_char.unwrap());
                } else {
                    write!(&mut s, "{}{}", counter, current_char.unwrap()).unwrap();
                }
            }
        } else {
            if counter == 1 {
                s.push(current_char.unwrap());
            } else {
                write!(&mut s, "{}{}", counter, current_char.unwrap()).unwrap();
            }
            current_char = Some(c);
            counter = 1;

            if idx == source.len() - 1 {
                s.push(current_char.unwrap());
            }
        }
    }

    s
}

pub fn decode(source: &str) -> String {
    unimplemented!("Return the run-length decoding of {}.", source);
}
