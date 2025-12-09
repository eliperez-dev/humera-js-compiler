use crate::token::{Token, SpannedToken};
use crate::lexer::Lexer;
use crate::ast::{Program, Statement, Expression, BinaryOp, UnaryOp};

pub struct Parser {
    lexer: Lexer,
    current_token: SpannedToken,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        Parser {
            lexer,
            current_token,
        }
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    fn consume_identifier(&mut self) -> String {
        match &self.current_token.token {
            Token::Identifier(s) => {
                let name = s.clone();
                self.advance();
                name
            }
            _ => self.error(format!("Expected identifier, found {:?}", self.current_token.token)),
        }
    }

    fn consume(&mut self, expected: Token) {
        if std::mem::discriminant(&self.current_token.token) == std::mem::discriminant(&expected) {
            self.advance();
        } else {
            self.error(format!("Expected {:?}, found {:?}", expected, self.current_token.token));
        }
    }

    fn error(&self, message: String) -> ! {
        panic!(
            "Error at line {}, column {}: {}",
            self.current_token.span.line,
            self.current_token.span.column,
            message
        );
    }

    pub fn parse_program(&mut self) -> Program {
        let mut body = Vec::new();
        while self.current_token.token != Token::EOF {
            body.push(self.parse_statement());
        }
        Program { body }
    }

    fn parse_statement(&mut self) -> Statement {
        match self.current_token.token {
            Token::Let => self.parse_variable_declaration(false),
            Token::Const => self.parse_variable_declaration(true),
            Token::Function => self.parse_function_declaration(),
            Token::If => self.parse_if_statement(),
            Token::While => self.parse_while_statement(),
            Token::Return => self.parse_return_statement(),
            Token::LBrace => {
                self.advance(); // consume '{'
                let block = self.parse_block();
                Statement::Block(block)
            }
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_variable_declaration(&mut self, is_const: bool) -> Statement {
        self.advance(); // consume 'let' or 'const'
        let name = self.consume_identifier();
        self.consume(Token::Eq);
        let init = self.parse_expression();
        self.consume(Token::Semi);
        Statement::VariableDeclaration { name, init, is_const }
    }

    fn parse_function_declaration(&mut self) -> Statement {
        self.advance(); // consume 'function'
        let name = self.consume_identifier();
        self.consume(Token::LParen);
        
        let mut params = Vec::new();
        if self.current_token.token != Token::RParen {
            loop {
                params.push(self.consume_identifier());
                if self.current_token.token == Token::Comma {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        self.consume(Token::RParen);
        self.consume(Token::LBrace);
        let body = self.parse_block();
        
        Statement::FunctionDeclaration { name, params, body }
    }

    fn parse_block(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();
        while self.current_token.token != Token::RBrace && self.current_token.token != Token::EOF {
            statements.push(self.parse_statement());
        }
        self.consume(Token::RBrace);
        statements
    }

    fn parse_if_statement(&mut self) -> Statement {
        self.advance(); // consume 'if'
        self.consume(Token::LParen);
        let condition = self.parse_expression();
        self.consume(Token::RParen);
        
        let then_branch = Box::new(self.parse_statement());
        let else_branch = if self.current_token.token == Token::Else {
            self.advance();
            Some(Box::new(self.parse_statement()))
        } else {
            None
        };

        Statement::If { condition, then_branch, else_branch }
    }

    fn parse_while_statement(&mut self) -> Statement {
        self.advance(); // consume 'while'
        self.consume(Token::LParen);
        let condition = self.parse_expression();
        self.consume(Token::RParen);
        let body = Box::new(self.parse_statement());
        Statement::While { condition, body }
    }

    fn parse_return_statement(&mut self) -> Statement {
        self.advance(); // consume 'return'
        let value = if self.current_token.token == Token::Semi {
            None
        } else {
            Some(self.parse_expression())
        };
        self.consume(Token::Semi);
        Statement::Return(value)
    }

    fn parse_expression_statement(&mut self) -> Statement {
        let expr = self.parse_expression();
        self.consume(Token::Semi);
        Statement::Expression(expr)
    }

    // Expression Parsing (Precedence Climbing)

    fn parse_expression(&mut self) -> Expression {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Expression {
        let expr = self.parse_equality();
        
        if self.current_token.token == Token::Eq {
            self.advance();
            let value = self.parse_assignment(); // Right-associative
            
            if let Expression::Identifier(name) = expr {
                return Expression::Assignment(name, Box::new(value));
            } else {
                self.error(format!("Invalid assignment target: {:?}", expr));
            }
        }
        
        expr
    }

    fn parse_equality(&mut self) -> Expression {
        let mut expr = self.parse_comparison();

        while matches!(self.current_token.token, Token::EqEq | Token::BangEq) {
            let op = match self.current_token.token {
                Token::EqEq => BinaryOp::Eq,
                Token::BangEq => BinaryOp::Ne,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_comparison();
            expr = Expression::Binary(Box::new(expr), op, Box::new(right));
        }
        expr
    }

    fn parse_comparison(&mut self) -> Expression {
        let mut expr = self.parse_term();

        while matches!(self.current_token.token, Token::Lt | Token::LtEq | Token::Gt | Token::GtEq) {
            let op = match self.current_token.token {
                Token::Lt => BinaryOp::Lt,
                Token::LtEq => BinaryOp::Le,
                Token::Gt => BinaryOp::Gt,
                Token::GtEq => BinaryOp::Ge,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_term();
            expr = Expression::Binary(Box::new(expr), op, Box::new(right));
        }
        expr
    }

    fn parse_term(&mut self) -> Expression {
        let mut expr = self.parse_factor();

        while matches!(self.current_token.token, Token::Plus | Token::Minus) {
            let op = match self.current_token.token {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Sub,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_factor();
            expr = Expression::Binary(Box::new(expr), op, Box::new(right));
        }
        expr
    }

    fn parse_factor(&mut self) -> Expression {
        let mut expr = self.parse_unary();

        while matches!(self.current_token.token, Token::Star | Token::Slash | Token::Percent) {
            let op = match self.current_token.token {
                Token::Star => BinaryOp::Mul,
                Token::Slash => BinaryOp::Div,
                Token::Percent => BinaryOp::Mod,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_unary();
            expr = Expression::Binary(Box::new(expr), op, Box::new(right));
        }
        expr
    }

    fn parse_unary(&mut self) -> Expression {
        if matches!(self.current_token.token, Token::Bang | Token::Minus) {
            let op = match self.current_token.token {
                Token::Bang => UnaryOp::Not,
                Token::Minus => UnaryOp::Neg,
                _ => unreachable!(),
            };
            self.advance();
            let right = self.parse_unary();
            return Expression::Unary(op, Box::new(right));
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Expression {
        match &self.current_token.token {
            Token::Number(n) => {
                let val = *n;
                self.advance();
                Expression::Number(val)
            }
            Token::Identifier(s) => {
                let name = s.clone();
                self.advance();
                
                if self.current_token.token == Token::LParen {
                    self.advance();
                    let mut args = Vec::new();
                    if self.current_token.token != Token::RParen {
                        loop {
                            args.push(self.parse_expression());
                            if self.current_token.token == Token::Comma {
                                self.advance();
                            } else {
                                break;
                            }
                        }
                    }
                    self.consume(Token::RParen);
                    Expression::Call(name, args)
                } else {
                    Expression::Identifier(name)
                }
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_expression();
                self.consume(Token::RParen);
                expr
            }
            _ => self.error(format!("Expected expression, found {:?}", self.current_token.token)),
        }
    }
}
