pub fn reply(message: &str) -> &str {
    match message.trim() {
        message if message.is_empty() =>"Fine. Be that way!",
        message if message.chars().filter(|c| c.is_alphabetic()).count() != 0 &&
            message.to_uppercase() == message => {

                if message.ends_with('?') {
                    "Calm down, I know what I'm doing!"
                } else {
                    "Whoa, chill out!"
                }
        }
        message if message.ends_with('?') => "Sure.",
        _ => "Whatever."
    }
}
