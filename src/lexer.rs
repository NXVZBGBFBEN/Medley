/*字句リスト*/
use std::char;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Times,
    Div,
    LParen,
    RParen,
}

/*字句解析器の構造定義*/
pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

/*字句解析器*/
impl Lexer {
    pub fn init(input: Vec<char>) -> Lexer {
        Lexer { input, position: 0 }
    }
    //token関数に構造体Lexerを渡す
    pub fn token(&mut self) -> Option<Token> {
        while self.curr().is_some() && self.curr().unwrap().is_whitespace() {
            self.next();
        }
        let token = if Self::is_number(self.curr()?) {
            //数字を読み込んだ場合
            let mut number = vec![*self.curr()?];
            while self.peek().is_some() && Self::is_number(self.peek().unwrap()) {
                self.next();
                number.push(*self.curr().unwrap());
            }
            String::from_iter(number).parse::<f64>().ok().map(Token::Number)
        } else if self.curr() == Some(&'\\') {
            //コマンドを読み込んだ場合
            let mut command = vec![*self.curr()?];
            while self.peek().is_some() && self.peek().unwrap().is_ascii_alphabetic() {
                self.next();
                command.push(*self.curr().unwrap());
            }
            match String::from_iter(command).as_str() {
                "\\times" => Some(Token::Times),
                "\\div" => Some(Token::Div),
                _ => None,
            }
        } else {
            //数字以外を読み込んだ場合
            match self.curr()? {
                '+' => Some(Token::Plus),
                '-' => Some(Token::Minus),
                '(' => Some(Token::LParen),
                ')' => Some(Token::RParen),
                _ => None,
            }
        };
        self.next();
        token
    }
    fn next(&mut self) {
        self.position += 1
    }
    fn curr(&mut self) -> Option<&char> {
        self.input.get(self.position)
    }
    fn peek(&mut self) -> Option<&char> {
        self.input.get(self.position + 1)
    }
    fn is_number(c: &char) -> bool {
        c.is_ascii_digit() || c == &'.'
    }
}