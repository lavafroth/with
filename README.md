# With

*Why say dash dash when `with ... as` do trick?*

A natural, speech transcription friendly way to specify command line arguments.

## How They Each Read

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

Sure it's more verbose but one of them is far easier to type with speech transcription.

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
