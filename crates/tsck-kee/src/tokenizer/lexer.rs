#![allow(unused)]

use crate::Kee;

#[derive(Debug)]
pub struct KeeFunc<'a> {
    pub kee: &'a str,
    pub func: &'a str,
}

#[derive(Debug, Clone)]
struct KeeFuncLexer<'a> {
    content: &'a str,
}

impl<'a> KeeFuncLexer<'a> {
    fn new(content: &'a str) -> Self {
        Self { content }
    }
    fn parse_line(&self, line: &'a str) -> KeeFunc<'a> {
        if let Some(pos) = line.find("=") {
            let kee = line[..pos].trim();
            let func = line[pos + 1..].trim();

            KeeFunc { kee, func }
        } else {
            KeeFunc {
                kee: "",
                func: line,
            }
        }
    }

    fn parse(&self) -> Vec<KeeFunc<'a>> {
        self.content
            .lines()
            .map(|line| self.parse_line(line))
            .collect()
    }
}

pub struct KeeParser<'a> {
    input: &'a str,
}

impl<'a> KeeParser<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }
    pub fn parse(&self) -> Vec<KeeFunc<'a>> {
        let lexer = KeeFuncLexer::new(self.input);
        lexer.parse()
    }
}

#[cfg(test)]
mod kee_lexer {
    use super::*;

    #[test]
    fn test_lexer() {
        let input = include_str!("../../kee.kee");
        let kfunc = KeeParser::new(input).parse();
        println!("Parsed {:#?}", kfunc);
        // if let Ok(part) = MaayApp::parse("ppp::FunctionCall(tesa)") {
        //     println!("{:?}", part);
        // }
    }
}
