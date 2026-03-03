pub fn recite(start_bottles: u32, take_down: u32) -> String {
    let mut s = String::new();

    for i in 0..take_down {
        if i != 0 {
            s.push_str("\n\n");
        }
        s.push_str(&paragraph(start_bottles - i));
    }

    s
}

fn paragraph(start_bottles: u32) -> String {
    let first_letter = letter(start_bottles);
    let next_letter = letter(start_bottles - 1);

    let first_line = format!("{} hanging on the wall,", first_letter);
    let last_line = format!(
        "There'll be {} hanging on the wall.",
        next_letter.to_ascii_lowercase()
    );

    format!(
        "{}\n{}\nAnd if one green bottle should accidentally fall,\n{}",
        first_line.clone(),
        first_line,
        last_line
    )
}

fn letter(number: u32) -> String {
    String::from(match number {
        10 => "Ten green bottles",
        9 => "Nine green bottles",
        8 => "Eight green bottles",
        7 => "Seven green bottles",
        6 => "Six green bottles",
        5 => "Five green bottles",
        4 => "Four green bottles",
        3 => "Three green bottles",
        2 => "Two green bottles",
        1 => "One green bottle",
        _ => "No green bottles",
    })
}
