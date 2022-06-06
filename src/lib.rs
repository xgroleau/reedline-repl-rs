//! reedline-repl-rs - [REPL](https://en.wikipedia.org/wiki/Read%E2%80%93eval%E2%80%93print_loop) library
//! for Rust
//!
//! # Example
//! ```
#![doc = include_str!("../examples/hello_world.rs")]
//! ```
//!
//! reedline-repl-rs uses the [builder](https://en.wikipedia.org/wiki/Builder_pattern) pattern extensively.
//! What these lines are doing is:
//! - creating a repl with an empty Context (see below)
//! - with a name of "MyApp", the given version, and the given description
//! - and adding a "hello" command which calls out to the `hello` callback function defined above
//! - the `hello` command has a single parameter, "who", which is required, and has the given help
//! message
//!
//! The `hello` function takes a reference to [ArgMatches](https://docs.rs/clap/latest/clap/struct.ArgMatches.html),
//! and an (unused) `Context`, which is used to hold state if you
//! need to - the initial context is passed in to the call to
//! [Repl::new](struct.Repl.html#method.new), in our case, `()`.
//! Because we're not using a Context, we need to include a generic type in our `hello` function,
//! because there's no way to pass an argument of type `()` otherwise.
//!
//! All command function callbacks return a `Result<Option<String>>`. This has the following
//! effect:
//! - If the return is `Ok(Some(String))`, it prints the string to stdout
//! - If the return is `Ok(None)`, it prints nothing
//! - If the return is an error, it prints the error message to stderr
//!
//! # Context
//!
//! The `Context` type is used to keep state between REPL calls. Here's an example:
//! ```
#![doc = include_str!("../examples/with_context.rs")]
//! ```
//! A few things to note:
//! - you pass in the initial value for your Context struct to the call to
//! [Repl::new()](struct.Repl.html#method.new)
//! - the context is passed to your command callback functions as a mutable reference
//!
//! # Help
//! reedline-repl-rs automatically builds help commands for your REPL with clap.
//!
//! ```bash
//! % myapp
//! MyApp> 〉help
//! MyApp v0.1.0: My very cool app
//!
//! COMMANDS:
//!     append     Append name to end of list
//!     help       Print this message or the help of the given subcommand(s)
//!     prepend    Prepend name to front of list
//!
//! MyApp> 〉help append
//! append
//! Append name to end of list
//!
//! USAGE:
//!     append <name>
//!
//! ARGS:
//!     <name>
//!
//! OPTIONS:
//!     -h, --help    Print help information
//! MyApp> 〉
//! ```
//!
//! # Errors
//!
//! Your command functions don't need to return `reedline_repl_rs::Error`; you can return any error from
//! them. Your error will need to implement `std::fmt::Display`, so the Repl can print the error,
//! and you'll need to implement `std::convert::From` for `reedline_repl_rs::Error` to your error type.
//! This makes error handling in your command functions easier, since you can just allow whatever
//! errors your functions emit bubble up.
//!
//! ```
#![doc = include_str!("../examples/custom_error.rs")]
//! ```

mod command;
mod completer;
mod error;
mod prompt;
mod repl;

pub use clap;
pub use crossterm;
pub use nu_ansi_term;
pub use reedline;

pub use error::{Error, Result};
#[doc(inline)]
pub use repl::Repl;

use clap::ArgMatches;

/// Command callback function signature
pub type Callback<Context, Error> =
    fn(&ArgMatches, &mut Context) -> std::result::Result<Option<String>, Error>;

/// Initialize the name, version and description of the Repl from your crate name, version and
/// description
#[macro_export]
macro_rules! initialize_repl {
    ($context: expr) => {{
        let repl = Repl::new($context)
            .with_name(crate_name!())
            .with_version(crate_version!())
            .with_description(crate_description!());

        repl
    }};
}
