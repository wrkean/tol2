use std::{cmp::Ordering, iter::Peekable, ops::Range, str::Chars};

use crate::{
    compiler::CompilerCtx,
    error::CompilerError,
    lexer::token::{Token, TokenKind},
};

pub mod token;

macro_rules! enter_bracket_and_add {
    ($lexer:expr, $ch:expr, $kind:expr) => {{
        $lexer.enter_bracket($ch);
        $lexer.add_token($kind, None);
    }};
}

macro_rules! exit_bracket_and_add {
    ($lexer:expr, $ch:expr, $kind:expr) => {{
        $lexer.exit_bracket($ch, 0..0)?;
        $lexer.add_token($kind, None);
    }};
}

pub struct Lexer<'a> {
    source_code: &'a str,
    source_iter: Peekable<Chars<'a>>,
    tokens: Vec<Token>,
    indent_stack: Vec<usize>,
    bracket_stack: Vec<char>,
    start: usize,
    current: usize,
    is_at_start: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(source_code: &'a str, _source_file_name: &str) -> Self {
        let source_iter = source_code.chars().peekable();

        Self {
            source_code,
            source_iter,
            tokens: Vec::new(),
            indent_stack: vec![0],
            bracket_stack: Vec::new(),
            start: 0,
            current: 0,
            is_at_start: true,
        }
    }

    pub fn lex(mut self, ctx: &mut CompilerCtx) -> Vec<Token> {
        while self.peek().is_some() {
            self.start = self.current;
            if self.is_at_start {
                self.handle_indentation()
                    .unwrap_or_else(|e| ctx.add_error(e));
                self.is_at_start = false;
                continue;
            }
            self.lex_token().unwrap_or_else(|e| ctx.add_error(e));
        }

        // Flush remaining indents
        while self.indent_stack.len() > 1 {
            self.indent_stack.pop();
            self.add_token(TokenKind::Dedent, Some("DEDENT"));
        }

        self.add_token(TokenKind::Eof, Some("<PAGTATAPOS_NG_FILE>"));
        self.tokens
    }

    fn handle_indentation(&mut self) -> Result<(), CompilerError> {
        if let Some('\n') = self.peek() {
            return Ok(());
        }

        if self.is_inside_bracket() {
            return Ok(());
        }

        let mut indent_count = 0;
        while let Some(c) = self.peek() {
            match c {
                ' ' => indent_count += 1,
                '\t' => indent_count += 4,
                _ => break,
            }

            self.advance();
        }

        let top_indent = *self.indent_stack.last().unwrap();
        match indent_count.cmp(&top_indent) {
            Ordering::Greater => {
                self.indent_stack.push(indent_count);
                self.add_token(TokenKind::Indent, Some("INDENT"));
            }
            Ordering::Less => {
                while let Some(&prev) = self.indent_stack.last() {
                    if indent_count < prev {
                        self.indent_stack.pop();
                        self.add_token(TokenKind::Dedent, Some("DEDENT"));
                    } else {
                        break;
                    }
                }

                if *self.indent_stack.last().unwrap() != indent_count {
                    return Err(CompilerError::InvalidDedent {
                        span: self.span().into(),
                    });
                }
            }
            Ordering::Equal => {}
        }

        Ok(())
    }

    fn lex_token(&mut self) -> Result<(), CompilerError> {
        let Some(ch) = self.advance() else {
            unreachable!()
        };
        match ch {
            '+' => self.add_token(TokenKind::Plus, None),
            '*' => self.add_token(TokenKind::Star, None),
            '/' => self.add_token(TokenKind::Slash, None),
            '(' => enter_bracket_and_add!(self, '(', TokenKind::LParen),
            ')' => exit_bracket_and_add!(self, ')', TokenKind::RParen),
            '{' => enter_bracket_and_add!(self, '{', TokenKind::LBrace),
            '}' => exit_bracket_and_add!(self, '}', TokenKind::RBrace),
            ',' => self.add_token(TokenKind::Comma, None),
            ':' => self.add_token(TokenKind::Colon, None),
            ';' => self.add_token(TokenKind::Semicolon, None),
            '"' => self.lex_string()?,
            '-' => {
                // Comments
                if self.match_char('-') {
                    while let Some(c) = self.advance() {
                        if c == '\n' {
                            break;
                        }
                    }
                } else {
                    self.add_token(TokenKind::Minus, None)
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenKind::EqualEqual, None);
                } else if self.match_char('>') {
                    self.add_token(TokenKind::Arrow, None);
                } else {
                    self.add_token(TokenKind::Equal, None);
                }
            }
            '|' => {
                if self.match_char('|') {
                    self.add_token(TokenKind::PipePipe, None);
                } else {
                    self.add_token(TokenKind::Pipe, None);
                }
            }
            '&' => {
                if self.match_char('&') {
                    self.add_token(TokenKind::AmperAmper, None);
                } else {
                    self.add_token(TokenKind::Amper, None);
                }
            }
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenKind::BangEqual, None);
                } else {
                    self.add_token(TokenKind::Bang, None);
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenKind::LessEqual, None);
                } else {
                    self.add_token(TokenKind::Less, None);
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenKind::GreaterEqual, None);
                } else {
                    self.add_token(TokenKind::Greater, None);
                }
            }
            '\r' | '\t' | ' ' => {} // Skip whitespace
            '\n' => {
                self.is_at_start = true;
                if !self.is_inside_bracket() {
                    let last_token = self.tokens.last().unwrap();

                    if last_token.kind.is_semicolon_inferrable() {
                        self.add_token(TokenKind::Semicolon, Some(";"));
                    }
                }
            }
            _ => {
                if ch.is_alphabetic() || ch == '_' {
                    self.lex_ident_or_keyword();
                } else if ch.is_numeric() {
                    if ch == '0' {
                        let lexing_mode = match self.peek() {
                            Some('b') => NumberLexingMode::Binary,
                            Some('x') => NumberLexingMode::Hexal,
                            Some('o') => NumberLexingMode::Octal,
                            _ => NumberLexingMode::Normal,
                        };

                        self.lex_number(lexing_mode);
                    } else {
                        self.lex_number(NumberLexingMode::Normal);
                    }
                }
            }
        };

        Ok(())
    }

    fn lex_string(&mut self) -> Result<(), CompilerError> {
        let mut strn = String::from("\"");
        while let Some(ch) = self.advance() {
            match ch {
                '\n' => {
                    return Err(CompilerError::UnterminatedString {
                        span: (self.span().start..self.span().start + 1).into(),
                    });
                }
                '"' => {
                    strn.push('"');
                    break;
                }
                '\\' => {
                    let escape_start = self.current - 1;
                    strn.push(match self.advance() {
                        Some('n') => '\n',
                        Some('r') => '\r',
                        Some('\'') => '\'',
                        Some('\"') => '\"',
                        Some('\\') => '\\',
                        Some('t') => '\t',
                        Some('0') => '\0',
                        None => {
                            return Err(CompilerError::UnterminatedString {
                                span: (self.span().start..self.span().start + 1).into(),
                            });
                        }
                        _ => {
                            let escape_end = self.current;
                            self.synchronize('"');
                            return Err(CompilerError::InvalidEscapeCharacter {
                                span: (escape_start..escape_end).into(),
                            });
                        }
                    })
                }
                _ => strn.push(ch),
            }
        }

        self.add_token(TokenKind::String, Some(&strn));
        Ok(())
    }

    fn lex_ident_or_keyword(&mut self) {
        while let Some(ch) = self.peek() {
            if !ch.is_alphanumeric() && ch != '_' {
                println!("Not alphanumeric or _ = {ch}");
                break;
            }
            self.advance();
        }

        let lexed = &self.source_code[self.start..self.current];
        match TokenKind::from_keyword(lexed) {
            Some(k) => self.add_token(k, Some(lexed)),
            None => self.add_token(TokenKind::Identifier, Some(lexed)),
        }
    }

    fn lex_number(&mut self, lexing_mode: NumberLexingMode) {
        match lexing_mode {
            NumberLexingMode::Normal => self.lex_normal_number(),
            NumberLexingMode::Binary => self.lex_binary(),
            NumberLexingMode::Hexal => self.lex_hex(),
            NumberLexingMode::Octal => self.lex_oct(),
        }
    }

    fn lex_normal_number(&mut self) {
        let mut is_float = false;
        while let Some(ch) = self.peek() {
            if !ch.is_numeric() && !matches!(ch, '.' | '_') {
                println!("Not numeric, _, or . = {ch}");
                break;
            }

            if ch == '.' {
                is_float = true;
            }

            self.advance();
        }

        let lexed = &self.source_code[self.start..self.current];
        let number_without_underscores: String = lexed.chars().filter(|&c| c != '_').collect();

        if is_float {
            self.add_token(TokenKind::Float, Some(&number_without_underscores));
        } else {
            self.add_token(TokenKind::Integer, Some(&number_without_underscores));
        }
    }

    fn lex_binary(&mut self) {
        self.advance(); // Consumes 'b'

        while let Some(ch) = self.peek() {
            if !matches!(ch, '0' | '1' | '_') {
                println!("Not 0 or 1 = {ch}");
                break;
            }

            self.advance();
        }

        let without_underscores: String = self.source_code[self.start..self.current]
            .chars()
            .filter(|&c| c != '_')
            .collect();
        self.add_token(TokenKind::BinLiteral, Some(&without_underscores));
    }

    fn lex_hex(&mut self) {
        self.advance(); // Consumes 'x'

        while let Some(ch) = self.peek() {
            if !matches!(
                ch.to_ascii_lowercase(),
                '0' | '1'
                    | '2'
                    | '3'
                    | '4'
                    | '5'
                    | '6'
                    | '7'
                    | '8'
                    | '9'
                    | 'a'
                    | 'b'
                    | 'c'
                    | 'd'
                    | 'e'
                    | 'f'
                    | '_'
            ) {
                println!(
                    "Not '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f' or '_' = {ch}"
                );
                break;
            }

            self.advance();
        }

        let without_underscores: String = self.source_code[self.start..self.current]
            .chars()
            .filter(|&c| c != '_')
            .collect();
        self.add_token(TokenKind::HexLiteral, Some(&without_underscores));
    }

    fn lex_oct(&mut self) {
        self.advance(); // Consumes 'o'

        while let Some(ch) = self.peek() {
            if !matches!(
                ch.to_ascii_lowercase(),
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '_'
            ) {
                println!("Not '0', '1', '2', '3', '4', '5', '6', '7', or '_' = {ch}");
                break;
            }

            self.advance();
        }

        let without_underscores: String = self.source_code[self.start..self.current]
            .chars()
            .filter(|&c| c != '_')
            .collect();
        self.add_token(TokenKind::OctalLiteral, Some(&without_underscores));
    }

    fn add_token(&mut self, kind: TokenKind, lexeme: Option<&str>) {
        match lexeme {
            Some(s) => self.tokens.push(Token {
                kind,
                lexeme: s.to_string(),
                span: self.span(),
            }),
            None => self.tokens.push(Token {
                kind,
                lexeme: self.source_code[self.span()].to_string(),
                span: self.span(),
            }),
        }
    }

    fn enter_bracket(&mut self, bracket: char) {
        self.bracket_stack.push(bracket);
    }

    fn exit_bracket(&mut self, bracket: char, span: Range<usize>) -> Result<(), CompilerError> {
        let expected = match bracket {
            ')' => '(',
            '}' => '{',
            _ => panic!("Invalid bracket, only call this function upon lexing a closing bracket"),
        };

        if self.bracket_stack.is_empty() {
            return Err(CompilerError::UnmatchedDelimiter {
                delimiter: format!("{bracket}"),
                span: span.into(),
            });
        }

        if expected != self.bracket_stack.pop().unwrap() {
            Err(CompilerError::UnmatchedBracket {
                bracket,
                span: span.into(),
            })
        } else {
            Ok(())
        }
    }

    fn synchronize(&mut self, until: char) {
        while let Some(ch) = self.advance() {
            if ch == until {
                return;
            }
        }
    }

    fn is_inside_bracket(&self) -> bool {
        !self.bracket_stack.is_empty()
    }

    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.source_iter.next()
    }

    fn peek(&mut self) -> Option<char> {
        self.source_iter.peek().copied()
    }

    // Returns the span starting from `start` to `current`
    fn span(&self) -> Range<usize> {
        self.start..self.current
    }

    fn match_char(&mut self, matching: char) -> bool {
        if let Some(ch) = self.peek() {
            if ch == matching {
                self.advance();
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

enum NumberLexingMode {
    Normal,
    Binary,
    Hexal,
    Octal,
}
