use logos::{Lexer, Logos, Source};
use std::fmt::{Display, Formatter};

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
    #[regex("<p><a.*href=\"http://[^\"]\"[^>]>[^<]</a></p>", extract_link_info)]
    #[regex("<p><a.*name=\"[^\"]\".*href=\"http://[^\"]\">[^<]</a.*></p>", extract_link_info)]
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
    let slice = lex.slice();
    // let LinkUrl: url = ;
    // let LinkText: text = ;
    (url, text)
}
