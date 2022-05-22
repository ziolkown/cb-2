use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
pub enum C1Token {

    /* key words */
    #[token("bool")]
    KwBoolean, 

    #[token("do")]
    KwDo,

    #[token("else")]
    KwElse,

    #[token("float")]
    KwFloat,

    #[token("for")]
    KwFor,

    #[token("if")]
    KwIf,

    #[token("int")]
    KwInt,

    #[token("printf")]
    KwPrintf,

    #[token("return")]
    KwReturn,

    #[token("void")]
    KwVoid,

    #[token("while")]
    KwWhile,

    /* Operators */
    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Asterisk,

    #[token("/")]
    Slash,

    #[token("=")]
    Assign,

    #[token("==")]
    Eq,

    #[token("!=")]
    Neq,

    #[token("<")]
    Lss,

    #[token(">")]
    Grt,

    #[token("<=")]
    Leq,

    #[token(">=")]
    Geq,

    #[token("&&")]
    And,

    #[token("||")]
    Or,

    /* other token */
    #[token(",")]
    Comma,

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

    /* term variables */
    #[regex("[0-9]*")]
    ConstInt,

    #[regex(r"((([0-9]*\.[0-9]+)([eE]([+-])?[0-9])?)|([0-9]+[eE]([+-])?[0-9]+))")]
    ConstFloat,

    #[regex("true|false")]
    ConstBoolean,

    #[regex("\"[^\"]*\"")]
    ConstString,

    #[regex(r"[a-z][a-zA-Z_0-9]*")]
    Id,

    //annotation over multiple lines
    #[regex(r"/\* [^\*/]*\*/", logos::skip)]
    //annotation in one line
    #[regex(r"//[^\n]*", logos::skip)]
    //other useless chars
    #[regex(r"[ \t\n\f]+", logos::skip)]
    #[error]
    Error,
}
