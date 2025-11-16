use brief_diagnostic::Span;

/// Token with associated span
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }
}

/// Token kinds
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Keywords
    Int,
    Char,
    Str,
    Dub,
    Bool,
    If,
    Else,
    While,
    For,
    In,
    Break,
    Continue,
    Match,
    Case,
    Def,
    Ret,
    Cls,
    Obj,
    Const,
    Null,
    True,
    False,

    // Operators
    Plus,           // +
    Minus,          // -
    Star,           // *
    Slash,          // /
    Percent,        // %
    Pow,            // **
    Assign,         // =
    InitAssign,     // :=
    PlusAssign,     // +=
    MinusAssign,    // -=
    StarAssign,     // *=
    SlashAssign,    // /=
    PercentAssign,  // %=
    PowAssign,      // **=
    Inc,            // ++
    Dec,            // --
    Eq,             // ==
    Ne,             // !=
    Lt,             // <
    Le,             // <=
    Gt,             // >
    Ge,             // >=
    Not,            // !
    And,            // &&
    Or,             // ||
    Shr,            // >>
    Shl,            // <<
    BitAnd,         // &
    BitOr,          // |
    BitXor,         // ^
    BitNot,         // ~
    Question,       // ?
    Colon,          // :

    // Punctuation
    LeftParen,      // (
    RightParen,     // )
    LeftBracket,    // [
    RightBracket,   // ]
    LeftBrace,      // {
    RightBrace,     // }
    Comma,          // ,
    Semicolon,      // ;
    Dot,            // .
    Arrow,          // ->

    // Literals
    Integer(i64),
    Double(f64),
    Character(char),
    StrPart(String),        // Part of string literal (raw text)
    InterpIdent(String),    // &name
    InterpPath(String),     // &obj.field

    // Identifiers
    Identifier(String),

    // Special
    Newline,
    Indent,
    Dedent,
    Eof,
}

impl TokenKind {
    /// Check if this is a keyword
    pub fn is_keyword(s: &str) -> bool {
        matches!(
            s,
            "int" | "char"
                | "str"
                | "dub"
                | "bool"
                | "if"
                | "else"
                | "while"
                | "for"
                | "in"
                | "break"
                | "continue"
                | "match"
                | "case"
                | "def"
                | "ret"
                | "cls"
                | "obj"
                | "const"
                | "null"
                | "true"
                | "false"
        )
    }

    /// Convert keyword string to token kind
    pub fn from_keyword(s: &str) -> Option<TokenKind> {
        Some(match s {
            "int" => TokenKind::Int,
            "char" => TokenKind::Char,
            "str" => TokenKind::Str,
            "dub" => TokenKind::Dub,
            "bool" => TokenKind::Bool,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "while" => TokenKind::While,
            "for" => TokenKind::For,
            "in" => TokenKind::In,
            "break" => TokenKind::Break,
            "continue" => TokenKind::Continue,
            "match" => TokenKind::Match,
            "case" => TokenKind::Case,
            "def" => TokenKind::Def,
            "ret" => TokenKind::Ret,
            "cls" => TokenKind::Cls,
            "obj" => TokenKind::Obj,
            "const" => TokenKind::Const,
            "null" => TokenKind::Null,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            _ => return None,
        })
    }
}




