mod parser;
mod tokenizer;
pub use parser::*;
use tokenizer::Tokenizer;

pub fn parse(json: String) -> Root {
    let mut tokenizer_struct = Tokenizer::new(json);
    let tokens = tokenizer_struct.generate();
    let mut parser_struct = Parser::new(tokens);
    parser_struct.parse()
}
