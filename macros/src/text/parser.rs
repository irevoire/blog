#[derive(Debug)]
pub(super) enum Statement<'a> {
    Heading(Expression<'a>),
    Expression(Expression<'a>),
}

#[derive(Debug)]

pub(super) enum Expression<'a> {
    Bold(Box<Expression<'a>>),
    Italic(Box<Expression<'a>>),
    Underline(Box<Expression<'a>>),
    Simple(&'a str),
}

pub(super) fn parse(s: &str) -> impl Iterator<Item = Statement> {
    Parser::new(s)
}

pub(super) struct Parser<'a> {
    s: &'a str,
    current: usize,
}

const RESERVED: [&str; 4] = ["#", "*", "_", "\n\n"];

impl<'a> Parser<'a> {
    fn new(s: &'a str) -> Self {
        Self { s, current: 0 }
    }

    fn s(&self) -> &'a str {
        if self.current == self.s.len() {
            ""
        } else {
            &self.s[self.current..]
        }
    }

    fn skip_whitespace(&mut self) {
        if self.s().is_empty() {
            return;
        }
        self.current += self.s().chars().position(|c| !c.is_whitespace()).unwrap_or(self.s.len());
    }

    fn advance(&mut self) {
        if let Some(c) = self.peek() {
            self.current += c.len_utf8();
        }
    }

    fn peek(&self) -> Option<char> {
        self.s().chars().next()
    }

    fn matches(&mut self, s: &[&str]) -> bool {
        for s in s {
            if self.s().starts_with(s) {
                self.current += s.len();
                return true;
            }
        }
        false
    }

    fn consume(&mut self, s: &str) {
        if self.matches(&[s]) {
            self.current += s.len();
        } else {
            panic!("Expected: {}", s);
        }
    }

    fn parse_simple(&mut self) -> Expression<'a> {
        let s = self.s();
        println!("parsing simple: {} with current {}", s, self.current);
        let before = self.current;
        while !self.s().is_empty() {
            println!("current: {} {}", self.s(), self.current);
            if RESERVED.iter().any(|r| self.s().starts_with(r)) {
                break;
            }
            self.advance();
        }
        if self.current == self.s.len() {
            Expression::Simple(&s[before..])
        } else {
            Expression::Simple(&s[before..self.current])
        }
    }

    fn parse_bold(&mut self) -> Expression<'a> {
        let simple = self.parse_simple();
        self.consume("**");
        Expression::Bold(Box::new(simple))
    }

    fn parse_italic(&mut self) -> Expression<'a> {
        let simple = self.parse_simple();
        self.consume("_");
        Expression::Italic(Box::new(simple))
    }

    fn parse_underline(&mut self) -> Expression<'a> {
        let simple = self.parse_simple();
        self.consume("__");
        Expression::Underline(Box::new(simple))
    }

    fn parse_expression(&mut self) -> Expression<'a> {
        if self.matches(&["**"]) {
            self.parse_bold()
        } else if self.matches(&["__"]) {
            self.parse_underline()
        } else if self.matches(&["_"]) {
            self.parse_italic()
        } else {
            self.parse_simple()
        }
    }

    fn parse_statement(&mut self) -> Statement<'a> {
        if self.matches(&["# "]) {
            Statement::Heading(self.parse_expression())
        } else {
            Statement::Expression(self.parse_expression())
        }
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Statement<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.s().is_empty() {
            return None;
        }

        Some(self.parse_statement())
    }
}