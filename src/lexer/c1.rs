use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum C1Token {
    // TODO: Define variants and their token/regex
    #[token("bool")]
    KwBoolean, 

    #[token("return")]
    KwReturn,

    #[token("if")]
    KwIf,

    #[token("else")]
    KwElse,

    #[token("float")]
    KwFloat,

    #[token("=")]
    Assign,

    #[token(";")]
    Semicolon,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[token(",")]
    Comma,

    #[token("||")]
    Or,

    #[token("==")]
    Eq,

    #[token("!=")]
    Neq,

    #[token("do")]
    KwDo,

    #[token("+")]
    Plus,

    #[token("&&")]
    And,

    #[token("*")]
    Asterisk,
    
    #[token("void")]
    KwVoid,

    #[token("int")]
    KwInt,

    #[token("printf")]
    KwPrintf,

    #[token("while")]
    KwWhile,

    #[regex(r"[a-z][a-zA-Z_0-9]*")]
    Id,

    #[regex("[0-9]*")]
    ConstInt,

    #[regex("\"[^\"]*\"")]
    ConstString,

    #[regex(r"[0-9]*(\.|e)[0-9]*")]
    ConstFloat,

    #[regex("true|false")]
    ConstBoolean,

    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[regex(r"/\* [^\*/]*\*/", logos::skip)]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    #[regex(r"//[^\n]*", logos::skip)]
    #[error]
    Error,
}
