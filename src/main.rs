mod tokenizer;
use tokenizer::*;

mod interface;
use interface::*;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let parsed_args = Interface::parse_cli_args(args);
    let program = Interface::retrieve_program(parsed_args); 
    let tokenizer = Tokenizer::new(program).into_iter().peekable();

    for token in tokenizer {
        println!("{:?}", token);
    }
}
