use clap::{Arg, ArgMatches, Command};
use reedline_repl_rs::{Repl, Result};

// Write "Hello" with given name
fn hello<T>(args: &ArgMatches, _context: &mut T) -> Result<Option<String>> {
    Ok(Some(format!("Hello, {}", args.value_of("who").unwrap())))
}

const PROMPT: &str = "MyApp";

fn main() -> Result<()> {
    let mut repl = Repl::new(())
        .with_name("MyApp")
        .with_version("v0.1.0")
        .with_prompt(&PROMPT)
        .with_description("My very cool app")
        .with_banner("Welcome to MyApp")
        .add_command(
            Command::new("hello")
                .arg(Arg::new("who").required(true))
                .about("Greetings!"),
            hello,
        );
    repl.run()
}
