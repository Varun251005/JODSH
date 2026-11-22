use crate::lexer::WordSegment;
use std::env;

pub fn expand_word(segments: &[WordSegment]) -> String {
    let mut result = String::new();
    for seg in segments {
        match seg {
            WordSegment::Unquoted(s) => result.push_str(&expand_string(s)),
            WordSegment::DoubleQuoted(s) => result.push_str(&expand_string(s)),
            WordSegment::SingleQuoted(s) => result.push_str(s), // NO expansion
        }
    }
    result
}

fn expand_string(input: &str) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();
    
    while let Some(c) = chars.next() {
        if c == '$' {
            if let Some(&'{') = chars.peek() {
                chars.next(); // consume {
                let mut var_name = String::new();
                while let Some(nc) = chars.next() {
                    if nc == '}' { break; }
                    var_name.push(nc);
                }
                if let Ok(val) = env::var(&var_name) {
                    result.push_str(&val);
                }
            } else {
                let mut var_name = String::new();
                while let Some(&nc) = chars.peek() {
                    if nc.is_alphanumeric() || nc == '_' {
                        var_name.push(nc);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if !var_name.is_empty() {
                    if let Ok(val) = env::var(&var_name) {
                        result.push_str(&val);
                    }
                } else {
                    result.push('$');
                }
            }
        } else {
            result.push(c);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_string() {
        unsafe { std::env::set_var("TEST_VAR", "hello"); }
        assert_eq!(expand_string("a $TEST_VAR b"), "a hello b");
        assert_eq!(expand_string("a ${TEST_VAR}b"), "a hellob");
    }

    #[test]
    fn test_expand_word() {
        unsafe { std::env::set_var("TEST_VAR", "hello"); }
        let segments = vec![
            WordSegment::Unquoted("a ".to_string()),
            WordSegment::SingleQuoted("$TEST_VAR".to_string()),
            WordSegment::Unquoted(" ".to_string()),
            WordSegment::DoubleQuoted("$TEST_VAR".to_string()),
        ];
        assert_eq!(expand_word(&segments), "a $TEST_VAR hello");
    }
}
