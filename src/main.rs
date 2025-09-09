use crate::lexer::Lexer;

mod lexer;
mod token;

fn main() {
    let input = r#"
let five = 5;
let ten = 10;
let add = func(x, y) {
x != y;
};
let result = add(five, ten);
        "#;

    let lexer = Lexer::new(input.to_owned());
    for token in lexer {
        println!("{token:?}")
    }
}
