use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecialChar {
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    LessThan,
    GreaterThan,
    Slash,
    Backslash,
    Pipe,
    Ampersand,
    Asterisk,
    Plus,
    Minus,
    Equal,
    Percent,
    Dollar,
    Hash,
    At,
    Caret,
    Tilde,
    Underscore,
    Backtick,
    Quote,
    Apostrophe,
}

impl TryFrom<char> for SpecialChar {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '(' => Ok(Self::LeftParen),
            ')' => Ok(Self::RightParen),
            '[' => Ok(Self::LeftBracket),
            ']' => Ok(Self::RightBracket),
            '{' => Ok(Self::LeftBrace),
            '}' => Ok(Self::RightBrace),
            '<' => Ok(Self::LessThan),
            '>' => Ok(Self::GreaterThan),
            '/' => Ok(Self::Slash),
            '\\' => Ok(Self::Backslash),
            '|' => Ok(Self::Pipe),
            '&' => Ok(Self::Ampersand),
            '*' => Ok(Self::Asterisk),
            '+' => Ok(Self::Plus),
            '-' => Ok(Self::Minus),
            '=' => Ok(Self::Equal),
            '%' => Ok(Self::Percent),
            '$' => Ok(Self::Dollar),
            '#' => Ok(Self::Hash),
            '@' => Ok(Self::At),
            '^' => Ok(Self::Caret),
            '~' => Ok(Self::Tilde),
            '_' => Ok(Self::Underscore),
            '`' => Ok(Self::Backtick),
            '"' => Ok(Self::Quote),
            '\'' => Ok(Self::Apostrophe),
            _ => Err(()),
        }
    }
}

impl SpecialChar {
    pub(crate) const fn as_str(&self) -> &'static str {
        match self {
            Self::LeftParen => "(",
            Self::RightParen => ")",
            Self::LeftBracket => "[",
            Self::RightBracket => "]",
            Self::LeftBrace => "{",
            Self::RightBrace => "}",
            Self::LessThan => "<",
            Self::GreaterThan => ">",
            Self::Slash => "/",
            Self::Backslash => "\\",
            Self::Pipe => "|",
            Self::Ampersand => "&",
            Self::Asterisk => "*",
            Self::Plus => "+",
            Self::Minus => "-",
            Self::Equal => "=",
            Self::Percent => "%",
            Self::Dollar => "$",
            Self::Hash => "#",
            Self::At => "@",
            Self::Caret => "^",
            Self::Tilde => "~",
            Self::Underscore => "_",
            Self::Backtick => "`",
            Self::Quote => "\"",
            Self::Apostrophe => "'",
        }
    }
}

impl fmt::Display for SpecialChar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
