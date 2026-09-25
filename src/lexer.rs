#[derive(Debug, PartialEq)]
pub enum Token {
    Word(String),
    Pipe,
    RedirectOutput,
    RedirectAppend,
    RedirectInput,
    RedirectError,
    AndIf,
    OrIf,
    Semicolon,
    Background,
}

#[derive(Debug, PartialEq)]
pub enum LexerError {
    UnterminatedDoubleQuote,
    UnterminatedSingleQuote,
    IncompleteEscape,
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, LexerError> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
            continue;
        }

        if c == '|' {
            tokens.push(Token::Pipe);
            chars.next();
            continue;
        }

        let mut word = String::new();
        let mut in_word = true;

        while in_word {
            let Some(&c) = chars.peek() else { break };
            
            match c {
                ' ' | '\t' | '\n' => {
                    in_word = false;
                }
                '|' => {
                    in_word = false;
                    // Do not consume, outer loop will grab it
                }
                '\'' => {
                    chars.next(); // consume opening quote
                    let mut closed = false;
                    while let Some(sc) = chars.next() {
                        if sc == '\'' {
                            closed = true;
                            break;
                        }
                        word.push(sc);
                    }
                    if !closed {
                        return Err(LexerError::UnterminatedSingleQuote);
                    }
                }
                '"' => {
                    chars.next(); // consume opening quote
                    let mut closed = false;
                    while let Some(dc) = chars.next() {
                        if dc == '"' {
                            closed = true;
                            break;
                        }
                        word.push(dc);
                    }
                    if !closed {
                        return Err(LexerError::UnterminatedDoubleQuote);
                    }
                }
                '\\' => {
                    chars.next(); // consume backslash
                    if let Some(ec) = chars.next() {
                        word.push(ec);
                    } else {
                        return Err(LexerError::IncompleteEscape);
                    }
                }
                _ => {
                    word.push(c);
                    chars.next();
                }
            }
        }

        if !word.is_empty() {
            tokens.push(Token::Word(word));
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_command() {
        assert_eq!(tokenize("ls"), Ok(vec![Token::Word("ls".to_string())]));
    }

    #[test]
    fn test_arguments() {
        assert_eq!(
            tokenize("ls -la /tmp"),
            Ok(vec![
                Token::Word("ls".to_string()),
                Token::Word("-la".to_string()),
                Token::Word("/tmp".to_string())
            ])
        );
    }

    #[test]
    fn test_multiple_spaces() {
        assert_eq!(
            tokenize("echo     hello"),
            Ok(vec![
                Token::Word("echo".to_string()),
                Token::Word("hello".to_string())
            ])
        );
    }

    #[test]
    fn test_double_quotes() {
        assert_eq!(
            tokenize("echo \"hello world\""),
            Ok(vec![
                Token::Word("echo".to_string()),
                Token::Word("hello world".to_string())
            ])
        );
    }

    #[test]
    fn test_single_quotes() {
        assert_eq!(
            tokenize("echo 'hello world'"),
            Ok(vec![
                Token::Word("echo".to_string()),
                Token::Word("hello world".to_string())
            ])
        );
    }

    #[test]
    fn test_escaped_space() {
        assert_eq!(
            tokenize("echo hello\\ world"),
            Ok(vec![
                Token::Word("echo".to_string()),
                Token::Word("hello world".to_string())
            ])
        );
    }

    #[test]
    fn test_unterminated_quote() {
        assert_eq!(
            tokenize("echo \"hello"),
            Err(LexerError::UnterminatedDoubleQuote)
        );
    }

    #[test]
    fn test_incomplete_escape() {
        assert_eq!(
            tokenize("echo hello\\"),
            Err(LexerError::IncompleteEscape)
        );
    }

    #[test]
    fn test_pipe_with_spaces() {
        assert_eq!(
            tokenize("ls | grep src"),
            Ok(vec![
                Token::Word("ls".to_string()),
                Token::Pipe,
                Token::Word("grep".to_string()),
                Token::Word("src".to_string()),
            ])
        );
    }

    #[test]
    fn test_pipe_without_spaces() {
        assert_eq!(
            tokenize("ls|grep"),
            Ok(vec![
                Token::Word("ls".to_string()),
                Token::Pipe,
                Token::Word("grep".to_string()),
            ])
        );
    }

    #[test]
    fn test_multiple_pipes() {
        assert_eq!(
            tokenize("ls | grep | wc"),
            Ok(vec![
                Token::Word("ls".to_string()),
                Token::Pipe,
                Token::Word("grep".to_string()),
                Token::Pipe,
                Token::Word("wc".to_string()),
            ])
        );
    }
}
