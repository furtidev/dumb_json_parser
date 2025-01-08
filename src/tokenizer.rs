use std::iter;
use std::iter::{from_fn, Peekable};
use std::vec::IntoIter;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Quote,
    Comma,
    Colon,
    Str(String),
    Number(f64),
    Bool(bool),
    Null,
    Eof,
}

pub(crate) struct Tokenizer {
    input: Peekable<IntoIter<char>>,
    string_mode: bool,
    curr_char: Option<char>,
}

impl Tokenizer {
    pub(crate) fn new(input: String) -> Self {
        let input = input.chars().collect::<Vec<char>>().into_iter().peekable();
        Self {
            curr_char: None,
            string_mode: false,
            input,
        }
    }

    fn advance(&mut self) {
        self.curr_char = self.input.next();
    }

    fn make_string(&mut self, is_not_string: bool) -> String {
        let mut ident = String::new();
        let mut escape_mode = false;

        ident.push(self.curr_char.unwrap());

        while let Some(ch) = self.input.peek() {
            // if it's not a string, only care about alphabets,
            // this essentially parses `true`, `false` and `null` values.
            if is_not_string {
                if (*ch).is_alphabetic() {
                    ident.push(*ch);
                    self.advance();
                } else {
                    break;
                }
            } else {
                if *ch != '"' && !escape_mode {
                    // go to escape mode if `\` was detected.
                    if *ch == '\\' {
                        escape_mode = true;
                        self.advance();
                        continue;
                    }
                    ident.push(*ch);
                    self.advance();
                } else if escape_mode {
                    // parse JSON standard escape codes
                    match *ch {
                        '"' => ident.push(*ch),
                        '\\' => ident.push('\\'),
                        '/' => ident.push('/'),
                        'b' => ident.push('\u{0008}'),
                        'f' => ident.push('\u{000c}'),
                        'n' => ident.push('\u{000a}'),
                        'r' => ident.push('\u{000d}'),
                        't' => ident.push('\u{0009}'),
                        _ => {}
                    }
                    escape_mode = false;
                    self.advance();
                } else {
                    // non-escape quote detected, end string.
                    break;
                }
            }
        }

        return ident;
    }

    pub(crate) fn generate(&mut self) -> Vec<Token> {
        self.advance();

        let mut tokens: Vec<Token> = vec![];

        // None is treated as EOF
        while self.curr_char.is_some() {
            let ch = self.curr_char.unwrap();

            // if lexer is in string mode,
            // parse the string separately, since different rules apply.
            if self.string_mode && ch != '"' {
                let tok = Token::Str(self.make_string(false));
                tokens.push(tok);
                self.advance();
                continue;
            }

            match ch {
                '{' => tokens.push(Token::LeftBrace),
                '}' => tokens.push(Token::RightBrace),
                '[' => tokens.push(Token::LeftBracket),
                ']' => tokens.push(Token::RightBracket),
                '"' => {
                    self.string_mode = !self.string_mode;
                    tokens.push(Token::Quote);
                }
                ',' => tokens.push(Token::Comma),
                ':' => tokens.push(Token::Colon),
                '0'..='9' | '-' => {
                    let num_result: Result<f64, _> = iter::once(ch)
                        .chain(from_fn(|| {
                            self.input.by_ref().next_if(|s| {
                                s.is_ascii_digit()
                                    || *s == 'e'
                                    || *s == 'E'
                                    || *s == '-'
                                    || *s == '+'
                                    || *s == '.'
                            })
                        }))
                        .collect::<String>()
                        .parse();

                    match num_result {
                        Ok(n) => tokens.push(Token::Number(n)),
                        Err(_) => {}
                    }
                }
                't' | 'f' => {
                    let bool_value = self.make_string(true);
                    if bool_value == "true" {
                        tokens.push(Token::Bool(true))
                    } else if bool_value == "false" {
                        tokens.push(Token::Bool(false))
                    }
                }
                'n' => {
                    let value = self.make_string(true);
                    if value == "null" {
                        tokens.push(Token::Null);
                    }
                }
                _ => {}
            }
            self.advance();
        }
        tokens.push(Token::Eof);
        tokens
    }
}
