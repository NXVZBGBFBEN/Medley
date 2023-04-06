use crate::lexer;
use std::mem;

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
    token: Vec<Option<lexer::Token>>,
    position: usize,
}

/*構文解析器*/
impl Parser {
    //構文解析の実行(優先度がLowest以上の式の解析)
    pub fn parse(token: Vec<Option<lexer::Token>>) -> Option<Box<Expr>> {
        let mut target = Parser { token, position: 0 };
        target.parse_expression(Precedence::Lowest)
    }
    //式の解析
    fn parse_expression(&mut self, precedence: Precedence) -> Option<Box<Expr>> {
        //左辺の解析
        let mut now = self.parse_prefix()?;
        //右辺の優先度が基準優先度より高い場合に中置演算子式として解析
        while self.peek().is_some() && precedence < self.peek_precedence() {
            self.next();
            now = self.parse_infix(now)?;
        }
        Some(now)
    }
    //前置演算子式(マイナス，数値，括弧，引数，分数)の解析
    fn parse_prefix(&mut self) -> Option<Box<Expr>> {
        match self.curr()?.as_ref()? {
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
        match self.curr()? {
            Some(lexer::Token::Number(n)) => Some(Box::new(Expr::Number(*n))),
            _ => None,
        }
    }
    //括弧の解析
    fn parse_grouped_expression(&mut self) -> Option<Box<Expr>> {
        self.next();
        let expression = self.parse_expression(Precedence::Lowest);
        if self.discriminant(&lexer::Token::RParen) {
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
        if self.discriminant(&lexer::Token::RBrace) {
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
        match self.curr()?.as_ref()? {
            lexer::Token::Plus | lexer::Token::Minus | lexer::Token::Times | lexer::Token::Div => {
                self.parse_infix_expression(left)
            }
            _ => Some(left),
        }
    }
    //中置演算子式の解析
    fn parse_infix_expression(&mut self, left: Box<Expr>) -> Option<Box<Expr>> {
        let token = self.curr()?.as_ref()?;
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
    fn token_precedence(token: &lexer::Token) -> Precedence {
        match token {
            lexer::Token::Plus | lexer::Token::Minus => Precedence::Sum,
            lexer::Token::Div | lexer::Token::Times => Precedence::Product,
            _ => Precedence::Lowest,
        }
    }
    fn peek_precedence(&mut self) -> Precedence {
        return match self.peek() {
            Some(x) => Self::token_precedence(x.as_ref().unwrap()),
            None => Precedence::Lowest,
        };
    }
    //次のトークンが引数として与えられたトークンかどうか判別する
    fn discriminant(&mut self, token: &lexer::Token) -> bool {
        match self.peek() {
            Some(x) => mem::discriminant(x.as_ref().unwrap()) == mem::discriminant(token),
            None => false,
        }
    }
    fn next(&mut self) {
        self.position += 1
    }
    fn curr(&mut self) -> Option<&Option<lexer::Token>> {
        self.token.get(self.position)
    }
    fn peek(&mut self) -> Option<&Option<lexer::Token>> {
        self.token.get(self.position + 1)
    }
}

/*構文木の実行*/
pub fn eval(expr: &Expr) -> Result<f64, String> {
    match expr {
        Expr::Number(n) => Ok(*n),
        Expr::PrefixExpr { operator: _, right } => Ok(-eval(right)?),
        Expr::InfixExpr {
            left,
            operator,
            right,
        } => {
            let left = eval(left)?;
            let right = eval(right)?;
            match operator.as_str() {
                "Plus" => Ok(left + right),
                "Minus" => Ok(left - right),
                "Times" => Ok(left * right),
                "Div" => {
                    if left == 0 as f64 && right == 0 as f64 {
                        Err(String::from("indeterminate (divided by 0)"))
                    } else if right == 0 as f64 {
                        Err(String::from("incompatible (divided by 0)"))
                    } else {
                        Ok(left / right)
                    }
                }
                _ => Err(String::from("invalid operator")),
            }
        }
        Expr::Fraction {
            numerator,
            denominator,
        } => {
            let numerator = eval(numerator)?;
            let denominator = eval(denominator)?;
            if numerator == 0 as f64 && denominator == 0 as f64 {
                Err(String::from("indeterminate (denominator is 0)"))
            } else if denominator == 0 as f64 {
                Err(String::from("incompatible (denominator is 0)"))
            } else {
                Ok(numerator / denominator)
            }
        }
    }
}
