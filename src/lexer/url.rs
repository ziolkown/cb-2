use logos::{Lexer, Logos};//, Source};
use std::fmt::{Display, Formatter};
use std::string::String;

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
    #[regex(r#"<p><a[ \n\t\fa-zA-Z0-9=">,äüö:]*http[:/\-\.() \n\t\fa-zA-Z0-9=">,äüö]*</a[ \n\f\t]*></p>"#, extract_link_info)]
    Link((LinkUrl, LinkText)),

    // Catch any error
    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    #[regex("<!DOCTYPE[^>]*>", logos::skip)]
    #[token("<html>", logos::skip)]
    #[token("<head>", logos::skip)]
    #[regex("<meta[^>]*>", logos::skip)]
    #[regex("<title>[^>]*>", logos::skip)]
    #[token("</head>", logos::skip)]
    #[token("<body>", logos::skip)]
    #[regex("<h1><a[^>]>*[^<]*</a></h1>", logos::skip)]
    #[regex("<p>[^<]*</p>", logos::skip)]
    #[regex(r#"<p><a[ \n\t\fa-zA-Z0-9=">,äüö]*</a[ \n\f\t]*></p>"#, logos::skip)]
    #[token("</body>", logos::skip)]
    #[token("</html>", logos::skip)]
    Error,
}

#[derive(Logos, Debug, PartialEq)]
enum URLInfo {
    #[regex("http[^\"]*")]
    Url,

    #[error]
    #[regex(r#"<p><a[ \n\t]*"#, logos::skip)]
    #[regex(r#"[" ]*name="[a-zA-Z0-9 ]*"[ \n>]*"#, logos::skip)]
    #[regex("href=\"", logos::skip)]
    #[regex("\">", logos::skip)]
    #[regex(r#"</a[ \n\t]*></p>"#, logos::skip)]
    Error,
}

/// Extracts the URL and text from a string that matched a Link token
fn extract_link_info(lex: &mut Lexer<URLToken>) -> (LinkUrl, LinkText) {
    let mut lexer = URLInfo::lexer(lex.slice());
    let mut url = String::from("");
    let mut text = String::from("");
    while let Some(val) = lexer.next() {
        let token = (val, lexer.slice());
        if format!("{:?}", token.0) == "Url" {
            url = String::from(token.1);
        }
        if format!("{:?}", token.0) == "Error" {
            text.push_str(token.1);
        }
    }
    return (LinkUrl(url), LinkText(text))
}