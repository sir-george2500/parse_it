pub fn lex(input: &str) -> Result<(), &'static str> {
    let mut chars = input.chars().filter(|c| !c.is_whitespace());

    // Expect the first character to be '{'
    match chars.next() {
        Some('{') => {}
        _ => return Err("Invalid: Expected '{'"),
    }

    // Main loop to parse key-value pairs
    loop {
        match chars.next() {
            Some('"') => {
                // Move over the string of the Key
                while let Some(c) = chars.next() {
                    if c == '"' {
                        break; // End of key
                    }
                }
            }
            Some('}') => {
                break; // End of object
            }
            _ => return Err("Invalid: Expected '\"' at the start of key or '}'"),
        }

        // Expect a colon `:`
        match chars.next() {
            Some(':') => {}
            _ => return Err("Invalid: Expected ':' after key"),
        }

        // Expect and parse the value
        match chars.next() {
            Some('"') => {
                // Parse string value
                while let Some(c) = chars.next() {
                    if c == '"' {
                        break; // End of value
                    }
                }
            }
            Some('t') => {
                // Parse the boolean value `true`
                if chars.next() == Some('r')
                    && chars.next() == Some('u')
                    && chars.next() == Some('e')
                {
                    // success
                } else {
                    return Err("Invalid: Expected 'true'");
                }
            }
            Some('f') => {
                // Parse the boolean value `false`
                if chars.next() == Some('a')
                    && chars.next() == Some('l')
                    && chars.next() == Some('s')
                    && chars.next() == Some('e')
                {
                    // success
                } else {
                    return Err("Invalid: Expected 'false'");
                }
            }
            Some('n') => {
                // Parse the null value
                if chars.next() == Some('u')
                    && chars.next() == Some('l')
                    && chars.next() == Some('l')
                {
                    // success
                } else {
                    return Err("Invalid: Expected 'null'");
                }
            }
            Some(c) if c.is_digit(10) || c == '-' => {
                // Parse the numeric value (positive or negative)
                while let Some(c) = chars.next() {
                    if !c.is_digit(10) {
                        break;
                    }
                }
            }
            _ => return Err("Invalid: Expected a string, boolean, null, or number"),
        }

        // After parsing key-value, expect either a `}` to end the object or a comma `,` to continue
        match chars.next() {
            Some('}') => break,
            Some(',') => continue,
            _ => return Err("Invalid: Expected ',' or '}'"),
        }
    }

    // Ensure no more characters are left
    if chars.next().is_some() {
        return Err("Invalid: Extra characters found");
    }

    Ok(())
}

