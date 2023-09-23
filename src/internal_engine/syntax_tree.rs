/*構文木の定義*/
pub enum SyntaxTree {
    Number(f64),
    PrefixExpr {
        operator: String,
        right: Box<SyntaxTree>,
    },
    InfixExpr {
        left: Box<SyntaxTree>,
        operator: String,
        right: Box<SyntaxTree>,
    },
    Fraction {
        numerator: Box<SyntaxTree>,
        denominator: Box<SyntaxTree>,
    },
}
