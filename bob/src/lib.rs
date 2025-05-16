pub fn reply(message: &str) -> &str {
    let message = message.trim();
    match message {
        _ if message
            .chars()
            .filter(|v| v.is_whitespace())
            .collect::<String>()
            == message
            || message.is_empty() =>
        {
            "Fine. Be that way!"
        }
        _ if *message
            .chars()
            .collect::<Vec<_>>()
            .get(message.len() - 1)
            .unwrap()
            == '?'
            && message.chars().any(|v| v.is_alphabetic())
            && message
                .chars()
                .filter(|v| {
                    if v.is_alphabetic() {
                        v.is_uppercase()
                    } else {
                        true
                    }
                })
                .collect::<String>()
                == message =>
        {
            "Calm down, I know what I'm doing!"
        }
        _ if *message
            .chars()
            .collect::<Vec<_>>()
            .get(message.len() - 1)
            .unwrap()
            == '?' =>
        {
            "Sure."
        }
        _ if message.chars().any(|v| v.is_alphabetic())
            && message
                .chars()
                .filter(|v| {
                    if v.is_alphabetic() {
                        v.is_uppercase()
                    } else {
                        true
                    }
                })
                .collect::<String>()
                == message =>
        {
            "Whoa, chill out!"
        }
        _ => "Whatever.",
    }
}
