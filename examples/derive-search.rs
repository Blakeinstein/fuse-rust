//! Deriving `Fuseable` instead of implementing it by hand.
//!
//! Mark the searchable fields with `#[fuse]`; everything else on the struct is left
//! alone. Run with:
//!
//!   cargo run --example derive-search --features derive

use fuse_rust::{Fuse, FuseProperty, Fuseable};
use std::collections::HashSet;

/// Weighted fields. `author` counts for more than `title`, and the two
/// non-textual fields are simply not annotated, so the derive skips them.
#[derive(Debug, PartialEq, Eq, Hash, Fuseable)]
struct Book {
    #[fuse(weight = 0.3)]
    title: String,
    #[fuse(weight = 0.7)]
    author: String,
    isbn: u64,
    page_count: u32,
}

/// A bare `#[fuse]` gives the field the default weight of 1.0.
#[derive(Debug, Fuseable)]
struct Tag {
    #[fuse]
    name: String,
}

/// Borrowed fields work too, and so do the lifetimes they need.
#[derive(Debug, Fuseable)]
struct Quote<'a> {
    #[fuse]
    text: &'a str,
    source: &'a str,
}

fn book(title: &str, author: &str, isbn: u64) -> Book {
    Book {
        title: String::from(title),
        author: String::from(author),
        isbn,
        page_count: 0,
    }
}

/// The derive writes exactly the impl you would have written by hand.
fn derive_matches_a_handwritten_impl() {
    struct Manual {
        title: String,
        author: String,
    }

    impl Fuseable for Manual {
        fn properties(&self) -> Vec<FuseProperty> {
            vec![
                FuseProperty {
                    value: String::from("title"),
                    weight: 0.3,
                },
                FuseProperty {
                    value: String::from("author"),
                    weight: 0.7,
                },
            ]
        }

        fn lookup(&self, key: &str) -> Option<&str> {
            match key {
                "title" => Some(&self.title),
                "author" => Some(&self.author),
                _ => None,
            }
        }
    }

    let derived = [
        book("Old Man's War fiction", "John X", 1),
        book("Right Ho Jeeves", "P.D. Mans", 2),
    ];
    let manual = [
        Manual {
            title: String::from("Old Man's War fiction"),
            author: String::from("John X"),
        },
        Manual {
            title: String::from("Right Ho Jeeves"),
            author: String::from("P.D. Mans"),
        },
    ];

    let fuse = Fuse::default();
    assert_eq!(
        fuse.search_text_in_fuse_list("man", &derived),
        fuse.search_text_in_fuse_list("man", &manual),
        "the derived impl should score identically to the handwritten one"
    );

    println!("== derived impl matches the handwritten one ==");
    for result in fuse.search_text_in_fuse_list("man", &derived) {
        println!(
            "score: {:.6}  title: {}",
            result.score, derived[result.index].title
        );
    }
}

/// Unannotated fields take no part in the search. `isbn` here is a perfect
/// textual match for the query, and is still ignored.
fn unmarked_fields_are_ignored() {
    let books = [book("The Silmarillion", "J.R.R. Tolkien", 12345)];
    let fuse = Fuse::default();

    assert!(
        fuse.search_text_in_fuse_list("12345", &books).is_empty(),
        "isbn is not annotated, so it must not be searched"
    );

    println!("\n== unmarked fields are ignored ==");
    println!("searching \"12345\" against isbn 12345 -> no match, as intended");
}

/// Derived types work with every search entry point, including the
/// reference-returning ones over unordered collections.
fn derived_types_work_with_ref_search() {
    let mut books = HashSet::new();
    books.insert(book("Old Man's War fiction", "John X", 1));
    books.insert(book("Right Ho Jeeves", "P.D. Mans", 2));
    books.insert(book("The Silmarillion", "J.R.R. Tolkien", 3));

    let fuse = Fuse::default();
    let results = fuse.search_text_in_fuseable_refs("man", &books);

    assert_eq!(results[0].item.author, "P.D. Mans");

    println!("\n== derived type searched inside a HashSet ==");
    for result in &results {
        println!("score: {:.6}  title: {}", result.score, result.item.title);
    }
}

fn default_weight_and_borrowed_fields() {
    let fuse = Fuse::default();

    let tags = [
        Tag {
            name: String::from("fiction"),
        },
        Tag {
            name: String::from("non-fiction"),
        },
    ];
    let results = fuse.search_text_in_fuse_list("fic", &tags);
    assert_eq!(results.len(), 2);

    println!("\n== bare #[fuse], default weight 1.0 ==");
    for result in &results {
        println!(
            "score: {:.6}  name: {}",
            result.score, tags[result.index].name
        );
    }

    let quotes = [Quote {
        text: "Not all those who wander are lost",
        source: "J.R.R. Tolkien",
    }];
    let results = fuse.search_text_in_fuse_list("wander", &quotes);
    assert_eq!(results.len(), 1);

    println!("\n== borrowed str fields ==");
    let quote = &quotes[results[0].index];
    println!(
        "score: {:.6}  text: {}  (source: {}, not searched)",
        results[0].score, quote.text, quote.source
    );
}

fn main() {
    derive_matches_a_handwritten_impl();
    unmarked_fields_are_ignored();
    derived_types_work_with_ref_search();
    default_weight_and_borrowed_fields();
}
