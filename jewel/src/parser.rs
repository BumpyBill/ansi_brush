use std::{iter::Peekable, str::Chars};

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a String) -> Self {
        Self {
            chars: source.chars().peekable(),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(char) = self.chars.next() {
            match char {
                '[' => {
                    let mut expression = String::new();
                    while let Some(char) = self.chars.next() {
                        if char == ']' {
                            break;
                        }

                        expression.push(char);
                    }

                    Some(Token::Expression(expression))
                }
                '$' => {
                    let mut var = String::new();
                    while let Some(peek) = self.chars.peek() {
                        if *peek == ' ' {
                            break;
                        }

                        var.push(self.chars.next().unwrap());
                    }

                    Some(Token::Variable(var))
                }
                _ => Some(Token::Char(char)),
            }
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub enum Token {
    Expression(String), // [bold green]
    Char(char),         // H
    Variable(String),   // $cwd
}
