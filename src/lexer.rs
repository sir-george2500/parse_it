pub fn lex(input: &str) -> Result<(), &'static str> {
    let mut chars = input.chars().filter(|c| !c.is_whitespace());

    // Expect the first character to be a {
    match chars.next() {
        Some('{') => {}
        _ => return Err("Invalid: Expected '{'"),
    }

    // Expect the key to start with "
    loop {
        match chars.next() {
            Some('"') => {}
            _ => return Err("Invalid: Expected '\"' at the start of key"),
        }

        // move over the string of the Key
        while let Some(c) = chars.next() {
            if c == '"' {
                break;
            }
        }

        // Expect a colon `:`
        match chars.next() {
            Some(':') => {}
            _ => return Err("Invalid: Expected ':' after key"),
        }

        // Expect a string value (starting with `"`
        match chars.next() {
            Some('"') => {}
            _ => return Err("Invalid: Expected '\"' at the start of value"),
        }

        // Parse the value content until the next `"`
        while let Some(c) = chars.next() {
            if c == '"' {
                break;
            }
        }

        // After parsing key-value, expect either a `}` to end the object or a comma `,` to continue
        match chars.next() {
            Some('}') => break,
            Some(',') => continue,
            _ => return Err("Invalid: Expected ',' or '}'"),
        }
    }

    if chars.next().is_some() {
        return Err("Invalid : Extra Character was found");
    }
    Ok(())
}
