use std::collections::HashMap;

fn parse_puzzle(input: &str) -> (Vec<String>, String, HashMap<char, u8>) {
    let mut parts = input.split(" == ");
    let inputs_parts: Vec<String> = parts.next().unwrap().split(" + ")
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    let output = parts.next().unwrap().to_owned();

    let letters: HashMap<char, u8> = (output.clone() + &(inputs_parts.join("")).to_string())
        .chars()
        .map(|c| (c, 0))
        .collect();

    (inputs_parts, output, letters)
}

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let parsed_input = parse_puzzle(input);
    dbg!(&parsed_input);



    None
}
