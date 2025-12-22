use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip(r"[ \t\f\r]+"))]
#[logos(utf8 = true)]
pub enum Token {
    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Star,

    #[token("/")]
    Slash,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn lex_single_token() {
        let mut tokens = Token::lexer("+-*/");

        assert_eq!(tokens.next(), Some(Ok(Token::Plus)));
        assert_eq!(tokens.slice(), "+");

        assert_eq!(tokens.next(), Some(Ok(Token::Minus)));
        assert_eq!(tokens.slice(), "-");

        assert_eq!(tokens.next(), Some(Ok(Token::Star)));
        assert_eq!(tokens.slice(), "*");

        assert_eq!(tokens.next(), Some(Ok(Token::Slash)));
        assert_eq!(tokens.slice(), "/");

        assert_eq!(tokens.next(), None);
    }
}
