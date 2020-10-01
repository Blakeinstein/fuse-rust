<h1 align="center">Fuse-RS</h1>

## What is Fuse?
Fuse is a super lightweight library which provides a simple way to do fuzzy searching.

## Usage

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
dbg!(result);
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

books.iter().for_each(|&item| {
    let result = fuse.search(search_pattern.as_ref(), &item);
    dbg!(result);
});

// alternatively you can use fuse.search_text_in_iterable("Te silm", books.iter())
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
    
    results.iter().for_each(|result| 
        println!(r#"
            index: {}
            score: {}
            results: {:?}
            ---------------
        "#, result.index, result.score, result.results)
    );
}
```

Furthermore, you can add a chunk size to run this over multiple threads.

Currently, the chunk size is one, so the chunks of size 1 will be run on seperate threads.
```rust
    fuse.search_text_in_fuse_list_with_chunk_size("man", &books, 1, |x: FuseableSearchResult| {
        dbg!(x);
    });
```

#### Example 5

You can look into examples/chunk-search.rs for the source code, and can run the same with:

```shell
cargo run --example chunk-search
```

This searches for a text over a list of 100 items with a chunk size of 10.