#[derive(Debug, PartialEq)]
enum Token<'a> {
    Ident(&'a str),
    String(&'a str),
    Number(i64),
    DoubleColon,
    LParen,
    RParen,
    Comma,
}
#[derive(Debug, Clone)]
struct Lexer<'a> {
    input: &'a str,
    pos: usize,
}

#[derive(Debug)]
pub enum Expr<'a> {
    Ident(&'a str),
    String(&'a str),
    Number(i64),
    Tuple(Vec<Expr<'a>>),
    Call { name: &'a str, args: Vec<Expr<'a>> },
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
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
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.next_char();
            } else {
                break;
            }
        }
        let a = "";
        let start = self.pos;

        match self.next_char()? {
            ':' => {
                if self.peek() == Some(':') {
                    self.next_char();
                    Some(Token::DoubleColon)
                } else {
                    None
                }
            }
            '(' => Some(Token::LParen),
            ')' => Some(Token::RParen),
            ',' => Some(Token::Comma),

            c if c.is_ascii_digit() => {
                while let Some(c) = self.peek() {
                    if c.is_ascii_digit() {
                        self.next_char();
                    } else {
                        break;
                    }
                }
                let num = self.input[start..self.pos].parse().ok()?;
                Some(Token::Number(num))
            }

            c if c.is_alphabetic() || c == '_' => {
                while matches!(
                self.peek(),
                Some(c) if c.is_alphanumeric() || c == '_' || c=='-'
                ) {
                    self.next_char();
                }
                Some(Token::Ident(&self.input[start..self.pos]))
            }

            c if c == '\'' => {
                while let Some(c) = self.peek() {
                    if c != '\'' {
                        self.next_char();
                    } else {
                        break;
                    }
                }
                self.next_char()?;
                Some(Token::String(&self.input[start + 1..self.pos - 1]))
            }

            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Func<'a> {
    pub entry: &'a str,
    pub func: &'a str,
    pub args: Vec<Expr<'a>>,
}

fn parse_value<'a>(lexer: &mut Lexer<'a>) -> Option<Expr<'a>> {
    match lexer.next_token()? {
        Token::Number(n) => Some(Expr::Number(n)),
        Token::String(n) => Some(Expr::String(n)),

        Token::Ident(name) => {
            if let Some(Token::LParen) = lexer.peek_token() {
                lexer.next_token(); // consume '('
                let args = parse_arg_list(lexer)?;
                Some(Expr::Call { name, args })
            } else {
                Some(Expr::Ident(name))
            }
        }

        Token::LParen => {
            // tuple
            let mut items = Vec::new();

            loop {
                items.push(parse_value(lexer)?);

                match lexer.next_token()? {
                    Token::Comma => continue,
                    Token::RParen => break,
                    _ => return None,
                }
            }

            Some(Expr::Tuple(items))
        }

        _ => None,
    }
}

fn parse_args<'a>(lexer: &mut Lexer<'a>) -> Option<Vec<Expr<'a>>> {
    let mut args = Vec::new();

    if let Some(Token::LParen) = lexer.next_token() {
        loop {
            args.push(parse_value(lexer)?);

            match lexer.next_token()? {
                Token::Comma => continue,
                Token::RParen => break,
                _ => return None,
            }
        }
    }

    Some(args)
}
fn parse_arg_list<'a>(lexer: &mut Lexer<'a>) -> Option<Vec<Expr<'a>>> {
    let mut args = Vec::new();

    if let Some(Token::RParen) = lexer.peek_token() {
        lexer.next_token();
        return Some(args);
    }

    loop {
        args.push(parse_value(lexer)?);

        match lexer.next_token()? {
            Token::Comma => continue,
            Token::RParen => break,
            _ => return None,
        }
    }

    Some(args)
}
pub fn parse_func<'a>(input: &'a str) -> Option<Func<'a>> {
    let mut lexer = Lexer::new(input);
    let Token::Ident(namespace) = lexer.next_token()? else {
        return None;
    };
    let Token::DoubleColon = lexer.next_token()? else {
        return None;
    };
    let Token::Ident(action) = lexer.next_token()? else {
        return None;
    };
    let args = parse_args(&mut lexer)?;
    Some(Func {
        entry: namespace,
        func: action,
        args,
    })
}

#[cfg(test)]
mod test_lexer {
    use crate::lexer::parse_func;

    #[test]
    fn lexer() {
        let inputs = [
            "app::PHOTOSHOP",
            "app::TSOCKEE(TODO)",
            "workspace::TOGGLE",
            "workspace::CALC(12,14)",
            "workspace::RESIZE(WINDOW(12,14))",
            "window::TRANSFORM((300,300),(0,0))",
            "app::TSOCKEE(TODO)",
            "app::PHOTOSHOP",
            "app::TSOCKEE(TSOOGLE SOMETHING)",
            "app::CYCLE",
        ];

        for s in inputs {
            if let Some(cmd) = parse_func(s) {
                println!("{:#?}", cmd);
            }
        }
    }
    #[test]
    fn lexer_adv() {
        let input = [
            "app::SCRIPT('main')",
            "app::SCRIPT('some/main.js')",
            "app::SCRIPT('path/to/another jsfile.js',(10,20,(10,10)))",
        ];
        for s in input {
            println!("{:#?}", parse_func(s))
        }
    }
}
