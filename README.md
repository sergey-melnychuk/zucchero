zucchero
========

Minimal macro for globally shared, type-safe singleton.

```toml

[dependencies]
zucchero = "1"

```

```rust
#[derive(Clone, Default, Debug)]
struct State {
    answer: usize,
}

zucchero::init!(State, expose);

fn main() {
    expose(|state| state.answer = 42);

    let state = expose(|state| state.clone());
    println!("{state:?}");
}
```
