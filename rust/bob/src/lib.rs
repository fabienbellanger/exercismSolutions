pub fn reply(message: &str) -> &str {
    let message = message.trim();
    let has_letter = message.chars().any(|c| c.is_alphabetic());
    let is_question = message.ends_with('?');
    let is_upper = message.to_ascii_uppercase() == message && has_letter;
    let is_empty = message.is_empty();

    match (is_question, is_empty, is_upper) {
        (true, _, true) => "Calm down, I know what I'm doing!",
        (true, _, false) => "Sure.",
        (false, true, _) => "Fine. Be that way!",
        (false, false, true) => "Whoa, chill out!",
        _ => "Whatever.",
    }
}
