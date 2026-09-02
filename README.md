# With

*Why say dash dash when `with ... as` do trick?*

A natural, speech transcription friendly way to specify command line arguments.

> [!WARNING]  
> This library is currently in early development. Some more polish is needed for error types and input handling for production use.

## Readability Comparison

### `getopt`

```sh
tar --create --verbose --file archive.tgz archive/
```

### `withopt`

```sh
archive with create as true \
        with verbose as true \
        with file as archive.tgz \
        with operand as archive/
```

> Note: the above `archive` program doesn't really exist, it's just to demonstrate the differences.

## Getting Started

- Add `with` as a dependency.

```sh
cargo add --git https://github.com/lavafroth/with with
```

- Import the trait and the derive macro.
- Create your struct that will hold the command line arguments.
- Annotate the struct with the derive macro.

```rust
use with::{WithOpt, withopt};

#[derive(withopt, Debug)]
struct GreetingApp {
  name: String,
  n_times: u32,
  easter_egg_probability: f32,
}
```

- Call `parse_args` on the struct.

```rust
fn main() {
  let app = GreetingApp::parse_args().unwrap();
  println!("{:?}", app);
}
```

## Who Is Using It

Me, for now. Currently dogfooding the library to sand out edge cases. I'll maintain a list of repositories where it is used.

- [takeout-transactions](https;//github.com/lavafroth/takeout-transactions)
- _Your next project?_
