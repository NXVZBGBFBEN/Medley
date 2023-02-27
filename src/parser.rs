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
    Fraction {
        numerator: Box<Expr>,
        denominator: Box<Expr>,
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
    pub fn init(mut lexer: lexer::Lexer) -> Result<Parser, String> {
        let curr = lexer.token()?;
        let peek = lexer.token()?;
        Ok(Parser { lexer, curr, peek })
    }
    //全字句の解析(優先度がLowest以上の式の解析)
    pub fn parse(&mut self) -> Option<Box<Expr>> {
        self.parse_expression(Precedence::Lowest)
    }
    //式の解析
    fn parse_expression(&mut self, precedence: Precedence) -> Option<Box<Expr>> {
        //左辺の解析
        let mut now = self.parse_prefix()?;
        //右辺の優先度が基準優先度より高い場合に中置演算子式として解析
        while self.peek.is_some() && precedence < self.peek_precedence() {
            self.next();
            now = self.parse_infix(now)?;
        }
        Some(now)
    }
    //前置演算子式(マイナス，数値，括弧，引数，分数)の解析
    fn parse_prefix(&mut self) -> Option<Box<Expr>> {
        match self.curr.as_ref()? {
            lexer::Token::Minus => self.parse_minus(),
            lexer::Token::Number(_) => self.parse_number(),
            lexer::Token::LParen => self.parse_grouped_expression(),
            lexer::Token::Frac => self.parse_fraction(),
            _ => None,
        }
    }
    //マイナスの解析(演算子を-として右辺をparse_expression()で解析)
    fn parse_minus(&mut self) -> Option<Box<Expr>> {
        self.next();
        Some(Box::new(Expr::PrefixExpr {
            operator: "Minus".to_string(),
            right: self.parse_expression(Precedence::Prefix)?,
        }))
    }
    //数値の解析(Token::NumberをExpr::Numberに変換)
    fn parse_number(&mut self) -> Option<Box<Expr>> {
        match self.curr.borrow() {
            Some(lexer::Token::Number(n)) => Some(Box::new(Expr::Number(*n))),
            _ => None,
        }
    }
    //括弧の解析
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
    //引数の解析
    fn parse_arguments(&mut self) -> Option<Box<Expr>> {
        self.next();
        let expression = self.parse_expression(Precedence::Lowest);
        if self.is_peek(&lexer::Token::RBrace) {
            self.next();
            expression
        } else {
            None
        }
    }
    //分数の解析
    fn parse_fraction(&mut self) -> Option<Box<Expr>> {
        self.next();
        let numerator = self.parse_arguments()?;
        self.next();
        let denominator = self.parse_arguments()?;
        Some(Box::new(Expr::Fraction {
            numerator,
            denominator,
        }))
    }
    //中置演算子の解析
    fn parse_infix(&mut self, left: Box<Expr>) -> Option<Box<Expr>> {
        match self.curr.as_ref()? {
            lexer::Token::Plus | lexer::Token::Minus | lexer::Token::Times | lexer::Token::Div => {
                self.parse_infix_expression(left)
            }
            _ => Some(left),
        }
    }
    //中置演算子式の解析
    fn parse_infix_expression(&mut self, left: Box<Expr>) -> Option<Box<Expr>> {
        let token = self.curr.as_ref()?;
        let operator = format!("{:?}", token);
        //現在解析中の演算子の優先度を取得
        let precedence = Self::token_precedence(token);
        self.next();
        //右辺の解析
        //優先度大→式が入る
        //優先度同等以下→最初の項が入る
        let right = self.parse_expression(precedence)?;
        Some(Box::new(Expr::InfixExpr {
            left,
            operator,
            right,
        }))
    }
    fn next(&mut self) {
        self.curr = self.peek.clone();
        if let Ok(x) = self.lexer.token() {
            self.peek = x
        };
    }
    fn token_precedence(token: &lexer::Token) -> Precedence {
        match token {
            lexer::Token::Plus | lexer::Token::Minus => Precedence::Sum,
            lexer::Token::Div | lexer::Token::Times => Precedence::Product,
            _ => Precedence::Lowest,
        }
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
}