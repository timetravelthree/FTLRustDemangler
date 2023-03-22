const HASH_LEN: usize = 16;
static HASH_PREFIX: &'static str = "::h";

fn unescape(input: &mut &str, output: &mut String, sequence: &str, value: char) -> bool {
    /* if the sequence is present
     * push the corresponding value
     * onto the output and skip the value */

    if input.starts_with(sequence) {
        *input = &input[sequence.len()..];
        output.push(value);
        return true;
    }

    return false;
}
fn strip_symbol_prefix_legacy(sym: &str) -> Option<&str> {
    // Try to strip prefix "__ZN"
    if let Some(result) = sym.strip_prefix("__ZN") {
        return Some(result);
    }

    // Try to strip prefix "_ZN"
    if let Some(result) = sym.strip_prefix("_ZN") {
        return Some(result);
    }

    // Try to strip prefix "ZN"
    sym.strip_prefix("ZN")
}

fn rust_demangle_symbol_element_legacy(mut legacy_symbol_element: &str) -> String {
    let mut i: usize = 0;
    let mut output: String = String::new();
    let input: &mut &str = &mut legacy_symbol_element;
    let mut last_char = '\0';
    let mut c: char;

    while input.len() > 0 {
        c = input.as_bytes()[0] as char;
        match c {
            '$' => {
                if !(unescape(input, &mut output, "$C$", ',')
                    || unescape(input, &mut output, "$SP$", '@')
                    || unescape(input, &mut output, "$BP$", '*')
                    || unescape(input, &mut output, "$RF$", '&')
                    || unescape(input, &mut output, "$LT$", '<')
                    || unescape(input, &mut output, "$GT$", '>')
                    || unescape(input, &mut output, "$LP$", '(')
                    || unescape(input, &mut output, "$RP$", ')')
                    || unescape(input, &mut output, "$u20$", ' ')
                    || unescape(input, &mut output, "$u22$", '\"')
                    || unescape(input, &mut output, "$u27$", '\'')
                    || unescape(input, &mut output, "$u2b$", '+')
                    || unescape(input, &mut output, "$u3b$", ';')
                    || unescape(input, &mut output, "$u5b$", '[')
                    || unescape(input, &mut output, "$u5d$", ']')
                    || unescape(input, &mut output, "$u7b$", '{')
                    || unescape(input, &mut output, "$u7d$", '}')
                    || unescape(input, &mut output, "$u7e$", '~'))
                {
                    panic!("invalid legacy symbol element {}", legacy_symbol_element);
                }
            }
            '.' => {
                if input.as_bytes()[1] as char == '.' {
                    output.push_str("::");
                    *input = &input[2..];
                } else {
                    output.push('-');
                    *input = &input[1..];
                }
            }
            '_' => {
                if !((i == 0 || last_char == ':') && input.as_bytes()[1] as char == '$') {
                    output.push(c);
                }
                *input = &input[1..];
            }

            'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'j' | 'k' | 'l' | 'm' | 'n'
            | 'o' | 'p' | 'q' | 'r' | 's' | 't' | 'u' | 'v' | 'w' | 'x' | 'y' | 'z' | 'A' | 'B'
            | 'C' | 'D' | 'E' | 'F' | 'G' | 'H' | 'I' | 'J' | 'K' | 'L' | 'M' | 'N' | 'O' | 'P'
            | 'Q' | 'R' | 'S' | 'T' | 'U' | 'V' | 'W' | 'X' | 'Y' | 'Z' | '0' | '1' | '2' | '3'
            | '4' | '5' | '6' | '7' | '8' | '9' | ':' => {
                output.push(c);
                *input = &input[1..];
            }
            _ => panic!("Invalid character '{}'", c),
        }
        i += 1;
        last_char = c;
    }

    return output;
}

fn split_symbol_into_elements_legacy(legacy_symbol: &str) -> Vec<&str> {
    let mut c: char;
    let mut cursor: usize = 0;
    let mut i: usize = 0;
    let end = legacy_symbol.len() - (HASH_PREFIX.len() + HASH_LEN) - 1;

    let mut legacy_symbol_elements: Vec<&str> = Vec::new();

    while i < end {
        c = legacy_symbol.as_bytes()[i] as char;

        if c.is_numeric() {
            cursor *= 10;
            cursor += c.to_digit(10).unwrap() as usize;
            i += 1;
        } else {
            legacy_symbol_elements.push(&legacy_symbol[i..i + cursor]);

            /* if accumulated 0 panic, this is done to prevent loop */
            assert_ne!(cursor, 0, "Invalid legacy symbol 'ZN{}'", legacy_symbol);

            i += cursor;
            cursor = 0;
        }
    }

    return legacy_symbol_elements;
}

fn demangle_symbol_legacy(legacy_symbol: &str) -> String {
    /* check if last character is the closing 'E' */

    if !(legacy_symbol.len() > 1 && legacy_symbol.chars().last().unwrap() == 'E') {
        return legacy_symbol.to_string();
    }

    let legacy_symbol_stripped = strip_symbol_prefix_legacy(&legacy_symbol);
    let legacy_symbol_elements: Vec<&str>;

    /* check for the legacy prefix if it is not present
     * return original symbol */

    match legacy_symbol_stripped {
        Some(p) => legacy_symbol_elements = split_symbol_into_elements_legacy(p),
        None => return legacy_symbol.to_string(),
    }

    /* demangle every single element of the symbol
     * then join everything with '::' */

    let legacy_elements_demangled: Vec<String> = legacy_symbol_elements
        .iter()
        .map(|x| rust_demangle_symbol_element_legacy(x))
        .collect();

    return legacy_elements_demangled.join("::");
}

fn main() {
    /* take all the inputs from command line arguments */
    let rust_symbols_list = std::env::args().skip(1);

    for rust_symbol_value in rust_symbols_list {
        println!("{}", demangle_symbol_legacy(&rust_symbol_value));
    }
}
