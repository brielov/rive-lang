use std::{iter::Peekable, str::Chars};

use crate::token::{Span, Token, WithSpan};

/// A lexer that tokenizes source code into a stream of `WithSpan<Token>` items.
///
/// The `Lexer` processes a string input character by character, producing tokens
/// such as keywords, operators, literals, and identifiers. It maintains position
/// information for error reporting and skips whitespace between tokens.
///
/// Use `Lexer::new` to create an instance, then iterate over it to retrieve tokens.
pub struct Lexer<'a> {
    /// An iterator over the characters of the source string, with peeking capability.
    chars: Peekable<Chars<'a>>,
    /// The current position in the source string (byte offset).
    pos: usize,
    /// A reference to the original source string, used for slicing and parsing.
    source: &'a str,
}

impl<'a> Lexer<'a> {
    /// Creates a new `Lexer` instance for the given source string.
    ///
    /// Initializes the character iterator and sets the starting position to 0.
    pub fn new(source: &'a str) -> Self {
        Self {
            chars: source.chars().peekable(),
            pos: 0,
            source,
        }
    }

    /// Advances to the next character in the source and updates the position.
    ///
    /// Returns the character if available, or `None` if at the end of the input.
    /// Updates `pos` based on the UTF-8 length of the consumed character.
    fn next(&mut self) -> Option<char> {
        let op = self.chars.next();
        if let Some(ch) = op {
            self.pos += ch.len_utf8();
        }
        op
    }

    /// Consumes the next character if it satisfies the given predicate.
    ///
    /// Peeks at the next character, applies the predicate, and advances if it matches.
    /// Returns `true` if a character was consumed, `false` otherwise.
    fn consume_if<F>(&mut self, f: F) -> bool
    where
        F: Fn(char) -> bool,
    {
        if let Some(ch) = self.chars.peek() {
            if f(*ch) {
                self.next();
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Consumes characters while the predicate returns `true`, collecting them into a string.
    ///
    /// Returns the substring from the starting position to the current position.
    /// Useful for lexing identifiers, numbers, and comments.
    fn consume_while<F>(&mut self, x: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let start = self.pos;
        while let Some(&ch) = self.chars.peek() {
            if x(ch) {
                self.next().unwrap();
            } else {
                break;
            }
        }
        self.source[start..self.pos].to_string()
    }

    /// Consumes a character if it matches, returning one of two tokens based on the result.
    ///
    /// Used for disambiguating multi-character operators (e.g., `=` vs. `==`).
    fn either(&mut self, to_match: char, matched: Token, unmatched: Token) -> Option<Token> {
        if self.consume_if(|x| x == to_match) {
            return Some(matched);
        }
        Some(unmatched)
    }

    /// Skips all consecutive whitespace characters.
    ///
    /// Advances the position until a non-whitespace character is encountered.
    fn skip_whitespace(&mut self) {
        self.consume_while(|x| x.is_whitespace());
    }

    /// Lexes a number (integer or float), handling an optional negative sign.
    ///
    /// Starts with the given character `ch` (already consumed) and continues consuming
    /// digits. If a `.` is found, it lexes a float; otherwise, an integer.
    /// Returns `Token::Int` or `Token::Float`, or `None` if parsing fails.
    fn lex_number(&mut self, ch: char, is_negative: bool) -> Option<Token> {
        let start = self.pos - ch.len_utf8();
        self.consume_while(|x| x.is_digit(10));
        let is_float = self.consume_if(|x| x == '.');
        if is_float {
            self.consume_while(|x| x.is_digit(10));
            self.source
                .get(start..self.pos)?
                .parse::<f64>()
                .ok()
                .map(|x| Token::Float(if is_negative { -x } else { x }))
        } else {
            self.source
                .get(start..self.pos)?
                .parse::<i64>()
                .ok()
                .map(|x| Token::Int(if is_negative { -x } else { x }))
        }
    }

    /// Lexes a string literal enclosed in double quotes, handling escapes.
    ///
    /// Consumes characters until a closing `"` is found, processing escape sequences
    /// like `\n`. Returns `Token::String` on success or `Token::UnterminatedString` if unclosed.
    fn lex_string(&mut self) -> Option<Token> {
        let mut value = String::new();
        let mut escaped = false;

        while let Some(ch) = self.next() {
            if escaped {
                let escaped_char = match ch {
                    'n' => '\n',
                    'r' => '\r',
                    't' => '\t',
                    '\\' => '\\',
                    '"' => '"',
                    _ => {
                        value.push('\\'); // Keep the backslash as a normal character
                        ch // Add the unknown escape character as is
                    }
                };
                value.push(escaped_char);
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                return Some(Token::String(value));
            } else {
                value.push(ch);
            }
        }

        Some(Token::UnterminatedString)
    }

    /// Lexes a character literal enclosed in single quotes, handling escapes.
    ///
    /// Expects exactly one character (or an escape sequence) followed by a closing `'`.
    /// Returns `Token::Char` on success, or `Token::UnterminatedChar`/`Token::InvalidCharLiteral` on error.
    fn lex_char(&mut self) -> Option<Token> {
        let Some(mut ch) = self.next() else {
            return Some(Token::UnterminatedChar);
        };

        if ch == '\\' {
            ch = match self.next() {
                Some('n') => '\n',
                Some('r') => '\r',
                Some('t') => '\t',
                Some('\\') => '\\',
                Some('\'') => '\'',
                Some(other) => other, // Unknown escapes are treated literally
                None => return Some(Token::UnterminatedChar), // Unterminated escape
            };
        }

        // If another character is found before the closing single quote, it's invalid
        if let Some(next) = self.next() {
            if next != '\'' {
                return Some(Token::InvalidCharLiteral);
            }
            Some(Token::Char(ch))
        } else {
            Some(Token::UnterminatedChar)
        }
    }

    /// Lexes an identifier or keyword starting with the given character.
    ///
    /// Consumes alphanumeric characters and underscores, then checks if the result
    /// is a keyword. Returns a specific `Token` variant for keywords or `Token::Identifier` otherwise.
    fn lex_identifier(&mut self, ch: char) -> Option<Token> {
        let start = self.pos - ch.len_utf8();
        self.consume_while(|x| x.is_ascii_alphanumeric() || x == '_');

        let ident = self.source.get(start..self.pos)?;

        Some(match ident {
            "break" => Token::Break,
            "const" => Token::Const,
            "continue" => Token::Continue,
            "enum" => Token::Enum,
            "fn" => Token::Fn,
            "for" => Token::For,
            "let" => Token::Let,
            "loop" => Token::Loop,
            "match" => Token::Match,
            "mod" => Token::Mod,
            "mut" => Token::Mut,
            "proto" => Token::Proto,
            "pub" => Token::Pub,
            "struct" => Token::Struct,
            "use" => Token::Use,
            "while" => Token::While,
            "false" => Token::Bool(false),
            "true" => Token::Bool(true),
            _ => Token::Identifier(ident.to_string()), // Only allocates if not a keyword
        })
    }

    /// Lexes a comment, either single-line (`#...`) or multi-line (`#*...*#`).
    ///
    /// Returns `Token::Comment` with the content, or `Token::UnterminatedComment` if multi-line is unclosed.
    fn lex_comment(&mut self) -> Option<Token> {
        if self.consume_if(|x| x == '*') {
            // Multi-line comment
            let start = self.pos;
            let mut end = None; // Track the last valid comment position

            while let Some(&ch) = self.chars.peek() {
                let pos_before = self.pos;
                self.next(); // Consume current character

                if ch == '*' {
                    if let Some(&'#') = self.chars.peek() {
                        self.next(); // Consume '#'
                        end = Some(pos_before); // Store position *before* `*#`
                        break;
                    }
                }
            }

            if let Some(end_pos) = end {
                Some(Token::Comment(self.source[start..end_pos].to_string()))
            } else {
                // Unterminated comment
                Some(Token::UnterminatedComment(
                    self.source[start..self.pos].to_string(),
                ))
            }
        } else {
            // Single-line comment
            let start = self.pos;
            self.consume_while(|x| x != '\n');
            Some(Token::Comment(self.source[start..self.pos].to_string()))
        }
    }

    /// Lexes the next token from the input based on the current character.
    ///
    /// Dispatches to specialized methods for numbers, strings, identifiers, etc.,
    /// or returns simple tokens for punctuation and operators.
    fn lex(&mut self) -> Option<Token> {
        let ch = self.next()?;
        match ch {
            '(' => Some(Token::LParen),
            ')' => Some(Token::RParen),
            '*' => Some(Token::Star),
            '+' => Some(Token::Plus),
            ',' => Some(Token::Comma),
            '/' => Some(Token::Slash),
            ';' => Some(Token::Semicolon),
            '[' => Some(Token::LBracket),
            ']' => Some(Token::RBracket),
            '{' => Some(Token::LBrace),
            '}' => Some(Token::RBrace),
            '^' => Some(Token::Caret),
            '~' => Some(Token::Tilde),
            '%' => Some(Token::Percent),
            '&' => self.either('&', Token::And, Token::Amp),
            '|' => self.either('|', Token::Or, Token::Pipe),
            ':' => self.either(':', Token::DoubleColon, Token::Colon),
            '!' => self.either('=', Token::NotEq, Token::Bang),
            '=' => self.either('=', Token::EqEq, Token::Eq),
            '-' => {
                if self.consume_if(|x| x.is_digit(10)) {
                    self.lex_number(ch, true)
                } else if self.consume_if(|x| x == '>') {
                    Some(Token::Arrow)
                } else {
                    Some(Token::Minus)
                }
            }
            '<' => {
                if self.consume_if(|x| x == '=') {
                    Some(Token::Le)
                } else if self.consume_if(|x| x == '<') {
                    Some(Token::LShift)
                } else {
                    Some(Token::Lt)
                }
            }
            '>' => {
                if self.consume_if(|x| x == '=') {
                    Some(Token::Ge)
                } else if self.consume_if(|x| x == '>') {
                    Some(Token::RShift)
                } else {
                    Some(Token::Gt)
                }
            }
            '.' => {
                if self.consume_if(|x| x == '.') {
                    if self.consume_if(|x| x == '=') {
                        Some(Token::RangeInclusive)
                    } else {
                        Some(Token::RangeExclusive)
                    }
                } else {
                    Some(Token::Dot)
                }
            }
            '#' => self.lex_comment(),
            '"' => self.lex_string(),
            '\'' => self.lex_char(),
            '0'..='9' => self.lex_number(ch, false),
            'a'..='z' | 'A'..='Z' | '_' => self.lex_identifier(ch),
            _ => Some(Token::Unknown(ch)),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = WithSpan<Token>;

    /// Produces the next token in the stream, wrapped with its source span.
    ///
    /// Skips whitespace, lexes the next token, and attaches its start and end positions.
    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();
        let start = self.pos;
        let value = self.lex()?;
        let span = Span {
            start,
            end: self.pos,
        };
        Some(WithSpan { value, span })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lex<'a>(source: &'a str) -> Vec<Token> {
        let lexer = Lexer::new(source);
        let mut tokens: Vec<Token> = vec![];
        for token in lexer {
            tokens.push(token.value);
        }
        tokens
    }

    #[test]
    fn test_operators() {
        let tokens = lex("& && -> ! ^ = == >= > <= < << - != || % | + .. ..= >> / * ~");
        assert_eq!(
            tokens,
            vec![
                Token::Amp,
                Token::And,
                Token::Arrow,
                Token::Bang,
                Token::Caret,
                Token::Eq,
                Token::EqEq,
                Token::Ge,
                Token::Gt,
                Token::Le,
                Token::Lt,
                Token::LShift,
                Token::Minus,
                Token::NotEq,
                Token::Or,
                Token::Percent,
                Token::Pipe,
                Token::Plus,
                Token::RangeExclusive,
                Token::RangeInclusive,
                Token::RShift,
                Token::Slash,
                Token::Star,
                Token::Tilde
            ]
        )
    }

    #[test]
    fn test_punctuation() {
        let tokens = lex(": , . :: { [ ( ) ] } ;");
        assert_eq!(
            tokens,
            vec![
                Token::Colon,
                Token::Comma,
                Token::Dot,
                Token::DoubleColon,
                Token::LBrace,
                Token::LBracket,
                Token::LParen,
                Token::RParen,
                Token::RBracket,
                Token::RBrace,
                Token::Semicolon
            ]
        );
    }

    #[test]
    fn test_numbers() {
        let tokens = lex("10 -10 10.5 -10.5 11.");
        assert_eq!(
            tokens,
            vec![
                Token::Int(10),
                Token::Int(-10),
                Token::Float(10.5),
                Token::Float(-10.5),
                Token::Float(11.0)
            ]
        );
    }

    #[test]
    fn test_strings() {
        let tokens = lex(r#""hello" "world" "escaped \"quote\"" "new\nline""#);
        assert_eq!(
            tokens,
            vec![
                Token::String("hello".to_string()),
                Token::String("world".to_string()),
                Token::String("escaped \"quote\"".to_string()),
                Token::String("new\nline".to_string()),
            ]
        );
    }

    #[test]
    fn test_empty_string() {
        let tokens = lex(r#""""#);
        assert_eq!(tokens, vec![Token::String("".to_string())]);
    }

    #[test]
    fn test_string_with_escapes() {
        let tokens = lex(r#""line1\nline2" "tab\tseparated""#);
        assert_eq!(
            tokens,
            vec![
                Token::String("line1\nline2".to_string()),
                Token::String("tab\tseparated".to_string()),
            ]
        );
    }

    #[test]
    fn test_unterminated_string() {
        let tokens = lex(r#""missing end"#);
        assert_eq!(tokens, vec![Token::UnterminatedString]);
    }

    #[test]
    fn test_invalid_escape_sequence() {
        let tokens = lex(r#""invalid \q escape""#);
        assert_eq!(
            tokens,
            vec![Token::String("invalid \\q escape".to_string())],
            "Unknown escape sequences should be treated as literal characters"
        );
    }

    #[test]
    fn test_valid_chars() {
        let tokens = lex(r#"'A' '1' ' '"#);
        assert_eq!(
            tokens,
            vec![Token::Char('A'), Token::Char('1'), Token::Char(' ')]
        );
    }

    #[test]
    fn test_valid_escapes() {
        let tokens = lex(r#"'\n' '\t' '\'' '\\'"#);
        assert_eq!(
            tokens,
            vec![
                Token::Char('\n'),
                Token::Char('\t'),
                Token::Char('\''),
                Token::Char('\\')
            ]
        );
    }

    #[test]
    fn test_unterminated_char() {
        let tokens = lex(r#"'\n"#);
        assert_eq!(tokens, vec![Token::UnterminatedChar]);
    }

    #[test]
    fn test_invalid_char_literal() {
        let tokens = lex(r#"'AB'"#);
        assert_eq!(
            tokens,
            vec![Token::InvalidCharLiteral, Token::UnterminatedChar]
        );
    }

    #[test]
    fn test_identifiers() {
        let tokens = lex(
            "break const continue enum fn for let loop match mod mut proto pub struct use while ident",
        );
        assert_eq!(
            tokens,
            vec![
                Token::Break,
                Token::Const,
                Token::Continue,
                Token::Enum,
                Token::Fn,
                Token::For,
                Token::Let,
                Token::Loop,
                Token::Match,
                Token::Mod,
                Token::Mut,
                Token::Proto,
                Token::Pub,
                Token::Struct,
                Token::Use,
                Token::While,
                Token::Identifier("ident".into())
            ]
        );
    }

    #[test]
    fn test_booleans() {
        let tokens = lex("true false");
        assert_eq!(tokens, vec![Token::Bool(true), Token::Bool(false)]);
    }

    #[test]
    fn test_single_line_comment() {
        let tokens = lex("# This is a single-line comment");
        assert_eq!(
            tokens,
            vec![Token::Comment(" This is a single-line comment".into())]
        );
    }

    #[test]
    fn test_multi_line_comment() {
        let tokens = lex("#* This is\na multi-line\ncomment *#");
        assert_eq!(
            tokens,
            vec![Token::Comment(" This is\na multi-line\ncomment ".into())]
        );
    }
}
