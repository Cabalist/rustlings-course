pub fn arr() -> &'static str {
    let a = ["All work and no play makes Jack a dull boy"; 100];

    if a.len() >= 100 {
        return "Wow, that's a big array!"
    } else {
        return "Meh, I eat arrays like that for breakfast."
    }
}
