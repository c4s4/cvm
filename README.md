# Cvm

Rust [CVM](https://en.wikipedia.org/wiki/Count-distinct_problem) implementation library.

## Usage

To call CVM library from your Rust code, add this to your `Cargo.toml`:

```toml
[dependencies]
cvm = "0.1"
```

Then you can call CVM functions in your code like this:

```rust
use cvm::CVM;

fn main() {
    let mut cvm = CVM::new(0.01);
    cvm.add("hello");
    cvm.add("world");
    cvm.add("hello");
    println!("Count: {}", cvm.count());
}
```

*Enjoy!*
