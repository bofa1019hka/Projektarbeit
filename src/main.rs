use crate::ast::Exp;

mod ast;
mod tokenizer;
mod parser;


fn display(e: Option<Exp>){
    if e.is_none() {
        println!("nothing \n");
    }
    else{
        let str: String = e.unwrap().pretty() + "\n";
        println!("{}", str);
    }
}

fn parse_expression(expression: &str) {
    let parser = parser::Parser::new(expression).parse();
    display(parser);
}

fn test_parser_good(){

    parse_expression("1");

    parse_expression("1 + 0");

    parse_expression("1 + (0)");

    parse_expression("1 + 2 * 0");

    parse_expression("1 * 2 + 0");

    parse_expression("(1 * ( 1 + 2) * 0 )");

    parse_expression("(1 + 2) * 0 + 2");
}

fn test_parser(){
    test_parser_good();
}

fn main() {
    test_parser();
}