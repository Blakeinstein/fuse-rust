use fuse_rust::{Fuse, SearchResult};

fn main() {
    let fuse = Fuse::default();
    let books = ["The Silmarillion", "The Lock Artist", "The Lost Symbol"];

    let results = fuse.search_text_in_iterable("Te silm", books.iter());
    assert_eq!(
        results,
        vec!(
            SearchResult {
                index: 0,
                score: 0.14285714285714285,
                ranges: vec!((0..1), (2..8), (10..14)),
            },
            SearchResult {
                index: 2,
                score: 0.49857142857142855,
                ranges: vec!((0..1), (2..5), (6..10), (11..12), (14..15)),
            },
            SearchResult {
                index: 1,
                score: 0.5714285714285714,
                ranges: vec!((0..1), (2..5), (8..9), (11..15)),
            },
        ),
        "Iterable search returned incorrect results"
    );
    dbg!(results);
}
