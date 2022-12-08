<h1 align="center">Fuse-rust</h1>
<p align="center">
    <a href="https://crates.io/crates/fuse-rust"><img src="https://img.shields.io/crates/v/fuse-rust.svg"/></a>
    <img src="https://github.com/Blakeinstein/fuse-rust/workflows/CI/badge.svg" />
</p>

## What is Fuse?
Fuse is a super lightweight library which provides a simple way to do fuzzy searching.

Fuse-RS is a port of https://github.com/krisk/fuse-swift written purely in rust.

## Usage
An example of a real use case, a search bar made using [iced](https://github.com/iced-rs/iced) is also available.

Try it using 
```shell
cargo run --package search_bar
```

![Demo](/.github/Demo.gif)


> Check all available examples and their source code [here.](/examples/)

### Async
Use the feature flag "async" to also be able to use async functions.
```toml
fuse-rust = { version = ..., features = ["async"]}
```

#### Initializing

The first step is to create a fuse object, with the necessary parameters. Fuse::default, returns the following parameters.
```rust
Fuse::default() = Fuse{
    location: 0, // Approx where to start looking for the pattern
    distance: 100, // Maximum distance the score should scale to
    threshold: 0.6, // A threshold for guess work
    max_pattern_length: 32, // max valid pattern length
    is_case_sensitive: false,
    tokenize: false, // the input search text should be tokenized
}
```

#### Example 1

Simple search.
```shell
cargo run --example simple-search
```

```rust
let fuse = Fuse::default();
let text = "Old Man's War";
let search_text = "od mn war";

let result = fuse.search_text_in_string(search_text, text);
assert_eq!(result, Some(ScoreResult{
    score: 0.4444444444444444,
    ranges: vec!((0..1), (2..7), (9..13)),
}), "Simple search returned incorrect results");
```

#### Example 2

Search over a string iterable.
```shell
cargo run --example iterable-search
```

```rust
let fuse = Fuse::default();
let books = [
    "The Silmarillion",
    "The Lock Artist",
    "The Lost Symbol"
];

// Improve performance by creating the pattern before hand.
let search_pattern = fuse.create_pattern("Te silm");

let results = fuse.search_text_in_iterable("Te silm", books.iter());
assert_eq!(results, vec!(
    SearchResult{
        index: 0,
        score: 0.14285714285714285,
        ranges: vec!((0..1), (2..8), (10..14)),
    },
    SearchResult{
        index: 2,
        score: 0.49857142857142855,
        ranges: vec!((0..1), (2..5), (6..10), (11..12), (14..15)),
    },
    SearchResult{
        index: 1,
        score: 0.5714285714285714,
        ranges: vec!((0..1), (2..5), (8..9), (11..15)),
    },
), "Iterable search returned incorrect results");
```

#### Example 3

Search over a list of items implementing the Fuseable trait.

```shell
cargo run --example fuseable-search
```

```rust
struct Book<'a> {
    title: &'a str,
    author: &'a str,
}

impl Fuseable for Book<'_>{
    fn properties(&self) -> Vec<FuseProperty> {
        return vec!(
            FuseProperty{value: String::from("title"), weight: 0.3},
            FuseProperty{value: String::from("author"), weight: 0.7},
        )
    }

    fn lookup(&self, key: &str) -> Option<&str> {
        return match key {
            "title" => Some(self.title),
            "author" => Some(self.author),
            _ => None
        }
    }
}
fn main() {    
    let books = [
        Book{author: "John X", title: "Old Man's War fiction"},
        Book{author: "P.D. Mans", title: "Right Ho Jeeves"},
    ];
    
    let fuse = Fuse::default();
    let results = fuse.search_text_in_fuse_list("man", &books);
    
    assert_eq!(results, vec!(
        FusableSearchResult{
            index: 1,
            score: 0.015000000000000003,
            results: vec!(FResult{
                value: String::from("author"),
                score: 0.015000000000000003,
                ranges: vec!((5..8)),
            }),
        },
        FusableSearchResult{
            index: 0,
            score: 0.027999999999999997,
            results: vec!(FResult{
                value: String::from("title"),
                score: 0.027999999999999997,
                ranges: vec!((4..7)),
            })
        }
    ), "Fuseable Search returned incorrect results");
}
```

Furthermore, you can add a chunk size to run this over multiple threads.

Currently, the chunk size is one, so the chunks of size 1 will be run on seperate threads.
```rust
    fuse.search_text_in_fuse_list_with_chunk_size("man", &books, 1, |x: FuseableSearchResult| {
        dbg!(x);
    });
```

#### Example 4

You can look into examples/chunk-search.rs for the source code, and can run the same with:

```shell
cargo run --example chunk-search
```

This searches for a text over a list of 100 items with a chunk size of 10.

## Options

As given above, Fuse takes the following options

- `location`: Approximately where in the text is the pattern expected to be found. Defaults to `0`
- `distance`: Determines how close the match must be to the fuzzy `location` (specified above). An exact letter match which is `distance` characters away from the fuzzy location would score as a complete mismatch. A distance of `0` requires the match be at the exact `location` specified, a `distance` of `1000` would require a perfect match to be within `800` characters of the fuzzy location to be found using a 0.8 threshold. Defaults to `100`
- `threshold`: At what point does the match algorithm give up. A threshold of `0.0` requires a perfect match (of both letters and location), a threshold of `1.0` would match anything. Defaults to `0.6`
- `maxPatternLength`: The maximum valid pattern length. The longer the pattern, the more intensive the search operation will be. If the pattern exceeds the `maxPatternLength`, the `search` operation will return `nil`. Why is this important? [Read this](https://en.wikipedia.org/wiki/Word_(computer_architecture)#Word_size_choice). Defaults to `32`
- `isCaseSensitive`: Indicates whether comparisons should be case sensitive. Defaults to `false`
