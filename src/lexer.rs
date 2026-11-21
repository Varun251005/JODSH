#[derive(Debug, PartialEq, Clone)]
pub enum WordSegment {
    Unquoted(String),
    SingleQuoted(String),
    DoubleQuoted(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Word(Vec<WordSegment>),
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

        if c == ';' {
            tokens.push(Token::Semicolon);
            chars.next();
            continue;
        }

        if c == '&' {
            chars.next();
            if let Some(&'&') = chars.peek() {
                tokens.push(Token::AndIf);
                chars.next();
            } else {
                tokens.push(Token::Background);
            }
            continue;
        }

        if c == '|' {
            chars.next();
            if let Some(&'|') = chars.peek() {
                tokens.push(Token::OrIf);
                chars.next();
            } else {
                tokens.push(Token::Pipe);
            }
            continue;
        }

        let mut segments = Vec::new();
        let mut in_word = true;
        let mut current_unquoted = String::new();

        let push_unquoted = |s: &mut String, segs: &mut Vec<WordSegment>| {
            if !s.is_empty() {
                segs.push(WordSegment::Unquoted(s.clone()));
                s.clear();
            }
        };

        while in_word {
            let Some(&c) = chars.peek() else { break };
            
            match c {
                ' ' | '\t' | '\n' => {
                    in_word = false;
                }
                '|' | '<' | '>' | '&' | ';' => {
                    in_word = false;
                }
                '\'' => {
                    push_unquoted(&mut current_unquoted, &mut segments);
                    chars.next(); // consume opening quote
                    let mut closed = false;
                    let mut sq = String::new();
                    while let Some(sc) = chars.next() {
                        if sc == '\'' {
                            closed = true;
                            break;
                        }
                        sq.push(sc);
                    }
                    if !closed {
                        return Err(LexerError::UnterminatedSingleQuote);
                    }
                    segments.push(WordSegment::SingleQuoted(sq));
                }
                '"' => {
                    push_unquoted(&mut current_unquoted, &mut segments);
                    chars.next(); // consume opening quote
                    let mut closed = false;
                    let mut dq = String::new();
                    while let Some(dc) = chars.next() {
                        if dc == '"' {
                            closed = true;
                            break;
                        }
                        dq.push(dc);
                    }
                    if !closed {
                        return Err(LexerError::UnterminatedDoubleQuote);
                    }
                    segments.push(WordSegment::DoubleQuoted(dq));
                }
                '\\' => {
                    chars.next(); // consume backslash
                    if let Some(ec) = chars.next() {
                        current_unquoted.push(ec);
                    } else {
                        return Err(LexerError::IncompleteEscape);
                    }
                }
                _ => {
                    current_unquoted.push(c);
                    chars.next();
                }
            }
        }

        push_unquoted(&mut current_unquoted, &mut segments);

        if !segments.is_empty() {
            tokens.push(Token::Word(segments));
        }
    }

    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn word(s: &str) -> Token {
        Token::Word(vec![WordSegment::Unquoted(s.to_string())])
    }
    
    fn dword(s: &str) -> Token {
        Token::Word(vec![WordSegment::DoubleQuoted(s.to_string())])
    }

    #[test]
    fn test_simple_command() {
        assert_eq!(tokenize("ls"), Ok(vec![word("ls")]));
    }

    #[test]
    fn test_double_quotes() {
        assert_eq!(
            tokenize("echo \"hello world\""),
            Ok(vec![
                word("echo"),
                dword("hello world")
            ])
        );
    }

    #[test]
    fn test_pipe_with_spaces() {
        assert_eq!(
            tokenize("ls | grep src"),
            Ok(vec![
                word("ls"),
                Token::Pipe,
                word("grep"),
                word("src"),
            ])
        );
    }

    #[test]
    fn test_redirects() {
        assert_eq!(
            tokenize("ls > output.txt"),
            Ok(vec![word("ls"), Token::RedirectOutput, word("output.txt")])
        );
        assert_eq!(
            tokenize("ls>>output.txt"),
            Ok(vec![word("ls"), Token::RedirectAppend, word("output.txt")])
        );
        assert_eq!(
            tokenize("cat < input.txt"),
            Ok(vec![word("cat"), Token::RedirectInput, word("input.txt")])
        );
        assert_eq!(
            tokenize("ls 2> errors.txt"),
            Ok(vec![word("ls"), Token::RedirectError, word("errors.txt")])
        );
    }
}
