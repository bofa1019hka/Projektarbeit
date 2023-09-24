pub enum Token {
    EOS,
    ZERO,
    ONE,
    TWO,
    OPEN,
    CLOSE,
    PLUS,
    MULT,
    DEFAULT,
}

pub struct Tokenizer {
    position: usize,
    expression: String,
    pub token: Token,
}

impl Tokenizer {
    pub fn new(text: &str) -> Tokenizer {
        let mut t = Tokenizer {
            position: 0,
            expression: text.to_string(),
            token: Token::DEFAULT,
        };
        t.token = t.next();
        t
    }

    pub fn next(&mut self) -> Token {
        if self.expression.len() <= self.position {
            return Token::EOS;
        }
        let my_vec: Vec<char> = self.expression.chars().collect();

        loop {
            match my_vec[self.position] {
                '0' => {
                    self.position += 1;
                    return Token::ZERO;
                }
                '1' => {
                    self.position += 1;
                    return Token::ONE;
                }
                '2' => {
                    self.position += 1;
                    return Token::TWO;
                }
                '(' => {
                    self.position += 1;
                    return Token::OPEN;
                }
                ')' => {
                    self.position += 1;
                    return Token::CLOSE;
                }
                '+' => {
                    self.position += 1;
                    return Token::PLUS;
                }
                '*' => {
                    self.position += 1;
                    return Token::MULT;
                }
                _ => {
                    self.position += 1;
                }
            }
        }
    }

    pub fn next_token(&mut self) {
        self.token = self.next();
    }
}
