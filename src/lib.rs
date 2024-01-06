use input::parse_input;
use runner::execute;

mod input;
mod program;
mod runner;

pub fn run(input: String) {
    let program = parse_input(input);
    execute(program);
}
