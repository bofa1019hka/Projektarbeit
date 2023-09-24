use crate::ast::Exp;
use crate::tokenizer::*;

pub struct Parser {
    t: Tokenizer,
}

impl Parser {
    pub fn new(expression: &str) -> Self {
        Self {
            t: Tokenizer::new(expression),
        }
    }

    pub fn parse(&mut self) -> Option<Exp> {
        self.parse_e()
    }

    // E  ::= T E'
    fn parse_e(&mut self) -> Option<Exp> {
        if let Some(t) = self.parse_t() {
            self.parse_e2(t)
        } else {
            None
        }
    }

    // E' ::= + T E' |
    fn parse_e2(&mut self, left: Exp) -> Option<Exp> {
        match self.t.token {
            Token::PLUS => {
                self.t.next_token();
                if let Some(right) = self.parse_t() {
                    self.parse_e2(Exp::Plus {
                        e1: Box::new(left),
                        e2: Box::new(right),
                        b: false,
                    })
                } else {
                    None
                }
            }
            _ => Some(left),
        }
    }

    // T  ::= F T'
    fn parse_t(&mut self) -> Option<Exp> {
        if let Some(f) = self.parse_f() {
            self.parse_t2(f)
        } else {
            None
        }
    }

    // T' ::= * F T' |
    fn parse_t2(&mut self, left: Exp) -> Option<Exp> {
        match self.t.token {
            Token::MULT => {
                self.t.next_token();
                if let Some(right) = self.parse_f() {
                    self.parse_t2(Exp::Mult {
                        e1: Box::new(left),
                        e2: Box::new(right),
                        b: false,
                    })
                } else {
                    None
                }
            }
            _ => Some(left),
        }
    }

    // F ::= N | (E)
    fn parse_f(&mut self) -> Option<Exp> {
        match &self.t.token {
            Token::ZERO => {
                self.t.next_token();
                Some(Exp::Int { val: 0, b: false })
            }
            Token::ONE => {
                self.t.next_token();
                Some(Exp::Int { val: 1, b: false })
            }
            Token::TWO => {
                self.t.next_token();
                Some(Exp::Int { val: 2, b: false })
            }
            Token::OPEN => {
                self.t.next_token();
                if let Some(e) = self.parse_e() {
                    match self.t.token {
                        Token::CLOSE => {
                            self.t.next_token();
                            Some(e)
                        }
                        _ => None,
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}