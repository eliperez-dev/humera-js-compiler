use crate::token::{Token, SpannedToken, Span};

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            pos: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn peek(&self) -> Option<char> {
        if self.pos >= self.input.len() {
            None
        } else {
            Some(self.input[self.pos])
        }
    }

    // Move forward one char
    fn advance(&mut self) -> Option<char> {
        let c = self.peek();
        if let Some(ch) = c {
            self.pos += 1;
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
        c
    }

    fn skip_whitespace(&mut self) {
        while let Some(char) = self.peek() {
            if char.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    pub fn next_token(&mut self) -> SpannedToken {
        self.skip_whitespace();

        let start_line = self.line;
        let start_column = self.column;

        let c = match self.advance() {
            Some(c) => c,
            None => return SpannedToken {
                token: Token::EOF,
                span: Span::new(start_line, start_column),
            },
        };

        let token = match c {
            // Single-char delimiters
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            ',' => Token::Comma,
            ';' => Token::Semi,
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '%' => Token::Percent,

            // Slash or Comment
            '/' => {
                if let Some('/') = self.peek() {
                    // It's a comment, skip until newline
                    while let Some(c) = self.peek() {
                        if c == '\n' { break; }
                        self.advance();
                    }
                    return self.next_token(); // Recursively get next real token
                } else {
                    Token::Slash
                }
            }
 
            // Multi-char operators
            '=' => if self.match_char('=') { Token::EqEq } else { Token::Eq },
            '!' => if self.match_char('=') { Token::BangEq } else { Token::Bang },
            '<' => if self.match_char('=') { Token::LtEq } else { Token::Lt },
            '>' => if self.match_char('=') { Token::GtEq } else { Token::Gt },

            // Numbers
            '0'..='9' => self.read_number(c),

            // Identifiers & Keywords
            'a'..='z' | 'A'..='Z' | '_' => self.read_identifier(c),

            _ => panic!("Unexpected character '{}' at line {}, column {}", c, start_line, start_column),
        };

        SpannedToken {
            token,
            span: Span::new(start_line, start_column),
        }
    }

    fn read_number(&mut self, first: char) -> Token {
        let mut s = String::new();
        s.push(first);
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                s.push(self.advance().unwrap());
            } else {
                break;
            }
        }
        Token::Number(s.parse().unwrap())
    }

    fn read_identifier(&mut self, first: char) -> Token {
        let mut s = String::new();
        s.push(first);
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                s.push(self.advance().unwrap());
            } else {
                break;
            }
        }

        match s.as_str() {
            "function" => Token::Function,
            "return" => Token::Return,
            "let" => Token::Let,
            "const" => Token::Const,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            _ => Token::Identifier(s),
        }
    }

    // Check if next char matches expected, consume if yes
    fn match_char(&mut self, expected: char) -> bool {
        if let Some(c) = self.peek() {
            if c == expected {
                self.advance();
                return true;
            }
        }
        false
    }
}
