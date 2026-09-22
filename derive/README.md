# fuse-rust-derive

Derive macro for the `Fuseable` trait of [fuse-rust](https://crates.io/crates/fuse-rust).

You normally do not depend on this crate directly. Enable the `derive` feature of
`fuse-rust` instead, which re-exports the macro:

```toml
[dependencies]
fuse-rust = { version = "0.4", features = ["derive"] }
```

## Usage

Mark each searchable field with `#[fuse]`. Fields without the attribute are ignored,
so a struct may hold ids, counts or anything else non-textual.

```rust
use fuse_rust::{Fuse, Fuseable};

#[derive(Fuseable)]
struct Book {
    #[fuse(weight = 0.3)]
    title: String,
    #[fuse(weight = 0.7)]
    author: String,
    isbn: u64, // not searched
}

let books = [
    Book { title: "Old Man's War fiction".into(), author: "John X".into(), isbn: 1 },
    Book { title: "Right Ho Jeeves".into(), author: "P.D. Mans".into(), isbn: 2 },
];

let results = Fuse::default().search_text_in_fuse_list("man", &books);
```

A bare `#[fuse]` uses the default weight of `1.0`:

```rust
# use fuse_rust::Fuseable;
#[derive(Fuseable)]
struct Tag {
    #[fuse]
    name: String,
}
```

## Rules

- Only structs with named fields are supported. Tuple structs have no field names to
  look up, and enums have no single set of fields.
- At least one field must be marked `#[fuse]`.
- `weight` must be greater than `0.0` and at most `1.0`. Weights are combined as
  `1.0 - weight`, so values outside that range score nonsensically.
- Every marked field must be usable as a `&str`; `String` and `&str` both work.
- Generic parameters and lifetimes on the struct are carried through to the impl.

## License

MIT, same as `fuse-rust`.
