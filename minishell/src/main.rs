mod minishell;
use std::env;

fn main() {
    if env::args().len() != 1 {
        println!("Usage: ./minishell");
        return;
    }
    minishell::minishell();
}
