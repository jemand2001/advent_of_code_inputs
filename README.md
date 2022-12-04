# `get_inputs`

This is a crate to automatically run your Advent of Code with your personal inputs.
To install it, add the following line to your `Cargo.toml` file:

```conf
get_inputs = {git = "https://github.com/jemand2001/advent_of_code_inputs"}
```

In your code, you should then be able to do the following:

```rs
use get_inputs::{run_on_input, Result};

// define parser and runner somehow

fn main() -> Result<()> {
    run_on_input(1, runner, parser);  // this will get your input, try to parse it using the parser, and call the runner on the result
}
```

Lastly, you need a file called `.env` in your working directory that contains your session cookie for [Advent of Code](https://adventofcode.com). It should look something like this:
```conf
session=1234567890abcdef...

```

Now you can run your Advent of Code solution with `cargo run`.
