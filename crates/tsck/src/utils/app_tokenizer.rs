#![allow(unused)]
#[derive(Debug, PartialEq)]
enum Token<'a> {
    Lhs(&'a str),
    Rhs(&'a str),
}

#[derive(Debug, Clone)]
struct Lexer<'a> {
    input: &'a str,
    pos: usize,
    returned_lhs: bool,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input,
            pos: 0,
            returned_lhs: false,
        }
    }

    fn peek(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn next_char(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.pos += ch.len_utf8();
        Some(ch)
    }

    fn peek_token(&self) -> Option<Token<'a>> {
        let mut clone = self.clone();
        clone.next_token()
    }

    fn next_token(&mut self) -> Option<Token<'a>> {
        if let Some(colon_pos) = self.input.find(':') {
            if !self.returned_lhs {
                self.returned_lhs = true;
                Some(Token::Lhs(&self.input[..colon_pos]))
            } else if self.pos == 0 {
                self.pos = self.input.len(); // Mark as consumed
                Some(Token::Rhs(&self.input[colon_pos + 1..]))
            } else {
                None
            }
        } else {
            // No colon, return entire string as LHS
            if self.pos == 0 {
                self.pos = self.input.len();
                Some(Token::Lhs(self.input))
            } else {
                None
            }
        }
    }
}
fn app_parser<'a>(input: &'a str) -> Option<(&'a str, &'a str)> {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.next_token();
    println!("{:#?},", tokens);
    None
}

#[cfg(test)]
mod test_tokenizer {
    use super::*;
    #[test]
    fn test_tokenizer() {
        let input = ["T:GOOGLE", "R:WEBSCKET*", "TSCK", "CR:WHOCH*", "TR:WWS*"];
        for s in input {
            let mut lexer = Lexer::new(s);
            while let Some(token) = lexer.next_token() {
                match token {
                    Token::Lhs(value) => println!("  LHS: {}", value),
                    Token::Rhs(value) => println!("  RHS: {}", value),
                }
            }
        }
    }
}
