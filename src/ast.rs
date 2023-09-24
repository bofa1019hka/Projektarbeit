pub enum Exp {
    Int {
        val: i32,
        b: bool
    },
    Plus {
        e1: Box<Exp>,
        e2: Box<Exp>,
        b: bool
    },
    Mult {
        e1: Box<Exp>,
        e2: Box<Exp>,
        b: bool
    },
}

impl Exp {
    pub fn eval(&mut self, set_to: bool) {
        match self {
            Exp::Int { ref mut b, .. } => {
                *b = set_to;
            }
            Exp::Plus { ref mut b, .. } => {
                *b = set_to;
            }
            _ => {}
        }
    }

    pub fn pretty(self) -> String {
        match self {
            Exp::Int { val, .. } => val.to_string(),
            Exp::Plus { e1, e2, b } => {
                if b {
                    format!("( {} + {} )", e1.pretty(), e2.pretty())
                } else {
                    format!("{} + {}", e1.pretty(), e2.pretty())
                }
            }
            Exp::Mult { mut e1, mut e2, .. } => {
                e1.eval(true);
                e2.eval(true);

                format!("{} * {}", e1.pretty(), e2.pretty())
            }
        }
    }
}
