use std::io;
use std::io::prelude::*;

mod object;
mod reader;
mod evaluator;

fn main() -> io::Result<()> {
    const PROMPT: &'static str = "> ";

    loop {
        print!("{}", PROMPT);
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        match reader::read(&input) {
            Ok(objects) => {
                objects.iter().for_each(|object| println!("{}", object));
            }
            Err(e) => println!("Something went wrong: {}", e),
        };
    }
}
