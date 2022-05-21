use logos::{Lexer, Logos, Source};
use std::fmt::{Display, Formatter};
use cb_2::lexer::url::Url::Url;

/// Tuple struct for link URLs
#[derive(Debug, PartialEq)]
pub struct LinkUrl(String);

/// Implement Display for printing
impl Display for LinkUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Tuple struct for link texts
#[derive(Debug, PartialEq)]
pub struct LinkText(String);

/// Implement Display for printing
impl Display for LinkText {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

/// Token enum for capturing of link URLs and Texts
#[derive(Logos, Debug, PartialEq)]
pub enum URLToken {
    #[regex("<p><a[ \n]*(name=\"[^\"]\"[ \n]*)?href=\"http://[^\"]\">[^<]</a[ \n]*></p>", extract_link_info)]
    Link((LinkUrl, LinkText)),

    #[regex("<!.*>")]
    #[token("<html>")]
    #[token("<head>")]
    #[regex("<meta.*>")]
    #[regex("<title>.*</title>")]
    #[token("</head>>")]
    #[token("<body>")]
    #[regex("<h1><a.*</a></h1>")]
    #[regex("<p>[^(<a)]</p>")]
    #[regex("<p><a name=\"[^\">]\">.*</a.*></p>")]
    #[token("</body>")]
    #[token("</html>")]
    Ignored,

    // Catch any error
    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

/// Extracts the URL and text from a string that matched a Link token
fn extract_link_info(lex: &mut Lexer<URLToken>) -> (LinkUrl, LinkText) {
    let url = extract_url(lex);
    let text = extract_text(lex);
    (url, text)
}

pub enum Url {
    #[regex("http://[^\"]*")]
    Url(LinkUrl),

    // #[regex(".*[^(http)]")]
    // #[regex("\"[ ]*name=[^(</p>]*</p>")]
    // Ignore,

    #[error]
    Error,
}

fn extract_url(lex: &mut Lexer<URLToken>) -> (LinkUrl) {
    let mut lexer = Url::lexer(lex.slice());
    let mut tokens = Vec::new();
    while let Some(val) = lexer.next() {
        tokens.push(val);
    }
    for val in tokens.ietr() {

    }
}

pub enum text {
    #[regex(">[^(</a)]*")]
    Text(LinkText),

    // #[regex()]
    // Ignore,

    #[error]
    Error,
}

fn extract_text(lex: &mut Lexer<URLToken>) -> (LinkText) {
    let mut lexer = text::lexer(lex.as_str());
    let mut tokens = Vec::new();
}