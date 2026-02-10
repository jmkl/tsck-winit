#![allow(unused)]
#[derive(Debug, Clone, PartialEq)]
enum FuncToken<'a> {
    Ident(&'a str),
    String(&'a str),
    Number(i64),
    LParen,
    RParen,
    Comma,
    DoubleColon,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FuncExpr<'a> {
    Number(i64),
    String(&'a str),
    TupleString(&'a str, &'a str),
    Ident(&'a str),

    Call {
        name: &'a str,
        args: Option<Box<FuncExpr<'a>>>,
    },
    Tuple(i64, i64),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Func<'a> {
    pub entry: &'a str,
    pub func: &'a str,
    pub args: Option<FuncExpr<'a>>,
}

#[derive(Debug, Clone)]
pub struct FuncLexer<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> FuncLexer<'a> {
    pub fn new(input: &'a str) -> Self {
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

    fn peek_token(&self) -> Option<FuncToken<'a>> {
        let mut clone = self.clone();
        clone.next_token()
    }

    fn next_token(&mut self) -> Option<FuncToken<'a>> {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.next_char();
            } else {
                break;
            }
        }

        let start = self.pos;

        match self.next_char()? {
            ':' => {
                if self.peek() == Some(':') {
                    self.next_char();
                    Some(FuncToken::DoubleColon)
                } else {
                    None
                }
            }
            '(' => Some(FuncToken::LParen),
            ')' => Some(FuncToken::RParen),
            ',' => Some(FuncToken::Comma),

            c if c.is_ascii_digit() => {
                while let Some(c) = self.peek() {
                    if c.is_ascii_digit() {
                        self.next_char();
                    } else {
                        break;
                    }
                }
                let num = self.input[start..self.pos].parse().ok()?;
                Some(FuncToken::Number(num))
            }

            c if c.is_alphabetic() || c == '_' => {
                while matches!(
                    self.peek(),
                    Some(c) if c.is_alphanumeric() || c == '_' || c == '-'
                ) {
                    self.next_char();
                }
                Some(FuncToken::Ident(&self.input[start..self.pos]))
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
                Some(FuncToken::String(&self.input[start + 1..self.pos - 1]))
            }

            _ => None,
        }
    }

    fn parse_expr_list<'b>(lexer: &mut FuncLexer<'b>) -> Option<FuncExpr<'b>> {
        let mut items = None;

        if matches!(lexer.peek_token(), Some(FuncToken::RParen)) {
            lexer.next_token();
            return None;
        }

        loop {
            items = FuncLexer::parse_expr(lexer);
            match lexer.next_token()? {
                FuncToken::Comma => continue,
                FuncToken::RParen => break,
                _ => return None,
            }
        }

        items
    }

    fn parse_expr<'b>(lexer: &mut FuncLexer<'b>) -> Option<FuncExpr<'b>> {
        let next_token = lexer.next_token()?;
        match next_token {
            FuncToken::Number(n) => Some(FuncExpr::Number(n)),
            FuncToken::String(s) => Some(FuncExpr::String(s)),
            FuncToken::Ident(name) => {
                if matches!(lexer.peek_token(), Some(FuncToken::LParen)) {
                    lexer.next_token();
                    let args = Box::new(FuncLexer::parse_expr_list(lexer)?);
                    Some(FuncExpr::Call {
                        name,
                        args: Some(args),
                    })
                } else {
                    Some(FuncExpr::Ident(name))
                }
            }

            FuncToken::LParen => {
                let mut items = Vec::with_capacity(2);
                if matches!(lexer.peek_token(), Some(FuncToken::RParen)) {
                    lexer.next_token();
                    return None;
                }
                loop {
                    let exp = FuncLexer::parse_expr(lexer)?;
                    items.push(exp);
                    match lexer.next_token()? {
                        FuncToken::Comma => continue,
                        FuncToken::RParen => break,
                        _ => return None,
                    }
                }

                match items.as_slice() {
                    [FuncExpr::Number(num1), FuncExpr::Number(num2)] => {
                        Some(FuncExpr::Tuple(*num1, *num2))
                    }
                    [FuncExpr::String(num1), FuncExpr::String(num2)] => {
                        Some(FuncExpr::TupleString(*num1, *num2))
                    }
                    _ => None,
                }
            }

            _ => None,
        }
    }

    pub fn parse_func<'b>(input: &'b str) -> Option<Func<'b>> {
        let mut lexer = FuncLexer::new(input);

        let FuncToken::Ident(entry) = lexer.next_token()? else {
            return None;
        };
        let FuncToken::DoubleColon = lexer.next_token()? else {
            return None;
        };
        let FuncToken::Ident(func) = lexer.next_token()? else {
            return None;
        };

        let args = if matches!(lexer.peek_token(), Some(FuncToken::LParen)) {
            lexer.next_token();
            FuncLexer::parse_expr_list(&mut lexer)
        } else {
            None
        };

        Some(Func { entry, func, args })
    }
}
