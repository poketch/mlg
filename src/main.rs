mod tokenizer;
mod interface;
use interface::*;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let parsed_args = Interface::parse_cli_args(args);
    let program = Interface::retrieve_program(parsed_args); 
    
    Interface::generate_html(program);

}
