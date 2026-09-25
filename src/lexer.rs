#[derive(Debug, PartialEq, Clone)]
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

        if c == '2' {
            let mut clone = chars.clone();
            clone.next();
            if let Some(&'>') = clone.peek() {
                tokens.push(Token::RedirectError);
                chars.next();
                chars.next();
                continue;
            }
        }

        if c == '>' {
            chars.next();
            if let Some(&'>') = chars.peek() {
                tokens.push(Token::RedirectAppend);
                chars.next();
            } else {
                tokens.push(Token::RedirectOutput);
            }
            continue;
        }

        if c == '<' {
            tokens.push(Token::RedirectInput);
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
                '|' | '<' | '>' => {
                    in_word = false;
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
    fn test_redirects() {
        assert_eq!(
            tokenize("ls > output.txt"),
            Ok(vec![Token::Word("ls".to_string()), Token::RedirectOutput, Token::Word("output.txt".to_string())])
        );
        assert_eq!(
            tokenize("ls>>output.txt"),
            Ok(vec![Token::Word("ls".to_string()), Token::RedirectAppend, Token::Word("output.txt".to_string())])
        );
        assert_eq!(
            tokenize("cat < input.txt"),
            Ok(vec![Token::Word("cat".to_string()), Token::RedirectInput, Token::Word("input.txt".to_string())])
        );
        assert_eq!(
            tokenize("ls 2> errors.txt"),
            Ok(vec![Token::Word("ls".to_string()), Token::RedirectError, Token::Word("errors.txt".to_string())])
        );
    }
}
