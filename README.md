<h1 align="center">vecmac</h1>

**vecmac** is a library that provides an experimental macro named `vecmac` to create a `vector`. It is similar to `vec!` macro of **standard library**.

```rust
use vecmac::vecmac;

let x: Vec<u32> = vecmac![36];

#[test]
fn test() {
    assert!(!x.is_empty());
    assert_eq!(x.len(), 1);
    assert_eq!(x[0], 36);
}
```

## How to use

```toml
[dependencies]
vecmac = { version = "0.1.0", url = git@github.com/royrustdev/vecmac.git}
```
