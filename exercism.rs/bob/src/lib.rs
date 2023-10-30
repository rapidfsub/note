pub fn reply(message: &str) -> &str {
    let message = message.trim();
    if message.is_empty() {
        "Fine. Be that way!"
    } else if is_yell(message) {
        if is_question(message) {
            "Calm down, I know what I'm doing!"
        } else {
            "Whoa, chill out!"
        }
    } else {
        if is_question(message) {
            "Sure."
        } else {
            "Whatever."
        }
    }
}

fn is_yell(message: &str) -> bool {
    message.chars().any(|c| c.is_alphabetic()) && message == message.to_ascii_uppercase()
}

fn is_question(message: &str) -> bool {
    message.ends_with("?")
}
