use std::borrow::Borrow;
use std::mem;
use crate::lexer;

/*構文木の定義*/
pub enum Expr {
    Number(f64),
    PrefixExpr {
        operator: String,
        right: Box<Expr>,
    },
    InfixExpr {
        left: Box<Expr>,
        operator: String,
        right: Box<Expr>,
    },
}

/*演算優先度の定義*/
#[derive(PartialOrd, PartialEq)]
enum Precedence {
    Lowest,
    Sum,
    Product,
    Prefix,
}

/*構文解析器の構造定義*/
pub struct Parser {
    lexer: lexer::Lexer,
    curr: Option<lexer::Token>,
    peek: Option<lexer::Token>,
}

/*構文解析器*/
impl Parser {
    pub fn init(mut lexer: lexer::Lexer) -> Parser {
        let curr = lexer.token();
        let peek = lexer.token();
        Parser { lexer, curr, peek }
    }
    fn next(&mut self) {
        self.curr = self.peek.clone();
        self.peek = self.lexer.token();
    }
    fn parse_prefix(&mut self) -> Option<Box<Expr>> {
        match self.curr.as_ref()? {
            lexer::Token::Minus => self.parse_minus(),
            lexer::Token::Number(_) => self.parse_number(),
            lexer::Token::LParen => self.parse_grouped_expression(),
            _ => None,
        }
    }
    fn parse_minus(&mut self) -> Option<Box<Expr>> {
        self.next();
        let number = self.parse_expression(Precedence::Prefix)?;
        Some(Box::new(Expr::PrefixExpr {
            operator: "Minus".to_string(),
            right: number,
        }))
    }
    fn parse_number(&mut self) -> Option<Box<Expr>> {
        match self.curr.borrow() {
            Some(lexer::Token::Number(n)) => Some(Box::new(Expr::Number(*n))),
            _ => None,
        }
    }
    fn token_precedence(token: &lexer::Token) -> Precedence {
        match token {
            lexer::Token::Plus | lexer::Token::Minus => Precedence::Sum,
            lexer::Token::Div | lexer::Token::Times => Precedence::Product,
            _ => Precedence::Lowest,
        }
    }
    pub fn parse(&mut self) -> Option<Box<Expr>> {
        self.parse_expression(Precedence::Lowest)
    }
    fn parse_expression(&mut self, precedence: Precedence) -> Option<Box<Expr>> {
        let mut left = self.parse_prefix()?;
        while self.peek.is_some() && precedence < self.peek_precedence() {
            self.next();
            left = self.parse_infix(left)?;
        }
        Some(left)
    }
    fn parse_infix(&mut self, left: Box<Expr>) -> Option<Box<Expr>> {
        let token = self.curr.as_ref()?;
        match token {
            lexer::Token::Plus | lexer::Token::Minus | lexer::Token::Times | lexer::Token::Div => {
                self.parse_infix_expression(left)
            }
            _ => Some(left),
        }
    }
    fn parse_infix_expression(&mut self, left: Box<Expr>) -> Option<Box<Expr>> {
        let token = self.curr.as_ref()?;
        let operator = format!("{:?}", token);
        let precedence = Self::token_precedence(token);
        self.next();
        let right = self.parse_expression(precedence)?;
        Some(Box::new(Expr::InfixExpr {
            left,
            operator,
            right,
        }))
    }
    fn is_peek(&self, token: &lexer::Token) -> bool {
        if self.peek.is_none() {
            return false;
        }
        mem::discriminant(self.peek.as_ref().unwrap()) == mem::discriminant(token)
    }
    fn peek_precedence(&self) -> Precedence {
        let token = self.peek.borrow();
        if token.is_none() {
            return Precedence::Lowest;
        }
        return Self::token_precedence(token.as_ref().unwrap());
    }
    fn parse_grouped_expression(&mut self) -> Option<Box<Expr>> {
        self.next();
        let expression = self.parse_expression(Precedence::Lowest);
        if self.is_peek(&lexer::Token::RParen) {
            self.next();
            expression
        } else {
            None
        }
    }
}