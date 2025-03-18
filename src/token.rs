#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Identifier(String),

    // Keywords
    Break,    // 'break'
    Const,    // 'const'
    Continue, // 'continue'
    Enum,     // 'enum'
    False,    // 'false'
    Fn,       // 'fn'
    For,      // 'for'
    Let,      // 'let'
    Loop,     // 'loop'
    Match,    // 'match'
    Mod,      // 'mod'
    Mut,      // 'mut'
    Proto,    // 'proto'
    Pub,      // 'pub'
    Struct,   // 'struct'
    True,     // 'true'
    Use,      // 'use'
    While,    // 'while'

    // Primitives
    Int(i64),       // 'int'
    Float(f64),     // 'float'
    String(String), // 'str'
    Char(char),     // 'char'
    Bool(bool),     // 'bool'

    // Operators & Punctuation
    Amp,            // '&'
    And,            // '&&'
    Arrow,          // '->',
    Bang,           // '!'
    Caret,          // '^'
    Colon,          // ':'
    Comma,          // ','
    Dot,            // '.'
    DoubleColon,    // '::'
    Eq,             // '=',
    EqEq,           // '==',
    Ge,             // '>=',
    Gt,             // '>
    LBrace,         // '{'
    LBracket,       // '['
    LParen,         // '('
    LShift,         // '<<'
    Le,             // '<=',
    Lt,             // '<',
    Minus,          // '-'
    NotEq,          // '!='
    Or,             // '||'
    Percent,        // '%'
    Pipe,           // '|'
    Plus,           // '+'
    RBrace,         // '}
    RBracket,       // ']
    RParen,         // ')
    RShift,         // '>>'
    RangeExclusive, // '..'
    RangeInclusive, // '..='
    Semicolon,      // ';
    Slash,          // '/'
    Star,           // '*'
    Tilde,          // '~',

    Comment(String),
    Unknown(char),
    UnterminatedString,
    UnterminatedChar,
    UnterminatedComment(String),
    InvalidCharLiteral, // More than one char in char literal
}

#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WithSpan<T> {
    pub value: T,
    pub span: Span,
}
