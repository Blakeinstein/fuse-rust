# Usage Examples

## Search Bar

Demo made using [iced-rs](https://github.com/iced-rs/iced).

You can run this example locally by running

```shell
cargo run --package search_bar
```

![Demo](/.github/Demo.gif)

#### Simple search

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

#### Iterable search

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

#### Fuseable search

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

#### Chunk search

You can look into chunk-search.rs for the source code, and can run the same with:

```shell
cargo run --example chunk-search
```