use crate::{Fuse, FuseProperty, Fuseable};

#[test]
fn multibyte_chars() {
    let pat = "f";
    let s = &[
        "®∮ℕ⊆ℕ₀⊂ℤℚ",
        "😊🥺😉😍😘😚",
        "⡍⠜⠇⠑⠹ ⠺⠁⠎",
        "გთხოვთ",
        "ıntəˈnæʃənəl",
        "γνωρίζω ἀπὸ",
        "コンニチハ",
    ];

    assert!(Fuse::default()
        .search_text_in_iterable(pat, s.iter())
        .is_empty());
}

#[test]
fn multibyte_chars_indices() {
    let needle = "f";
    let s = "®f∮";

    let fuse = Fuse::default();
    let pat = fuse.create_pattern(needle);
    let x = fuse.search(pat.as_ref(), s).unwrap();
    let r = &x.ranges[0];

    assert_eq!(&s[r.start..r.end], needle);
}

#[test]
fn full_match_higher() {
    let s = &["Syrup", "Syrup2", "Live", "Live2", "Live3"];

    let fuse = Fuse::default();

    let result1 = fuse.search_text_in_iterable("Syrup", s.iter());

    assert_eq!(result1.len(), 2);
    assert_eq!(result1[0].index, 0);
    assert_eq!(result1[1].index, 1);

    let result2 = fuse.search_text_in_iterable("live", s.iter());

    assert_eq!(result2.len(), 3);
    assert_eq!(result2[0].index, 2);
    assert_eq!(result2[1].score, result2[2].score);
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Book {
    title: String,
    author: String,
}

impl Fuseable for Book {
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

fn book(title: &str, author: &str) -> Book {
    Book {
        title: String::from(title),
        author: String::from(author),
    }
}

/// Three of these four match "man", so a chunk size of 1 spreads the matches
/// over several chunks, and the scores are distinct so the sort is deterministic.
fn books() -> Vec<Book> {
    vec![
        book("Old Man's War fiction", "John X"),
        book("Right Ho Jeeves", "P.D. Mans"),
        book("The Silmarillion", "J.R.R. Tolkien"),
        book("The Lock Artist", "Steve Hamilton"),
    ]
}

#[test]
fn blanket_impl_allows_searching_borrowed_items() {
    let books = books();
    let borrowed: Vec<&Book> = books.iter().collect();
    let fuse = Fuse::default();

    assert_eq!(
        fuse.search_text_in_fuse_list("man", &borrowed),
        fuse.search_text_in_fuse_list("man", &books)
    );
}

#[test]
fn fuseable_refs_matches_fuse_list() {
    let books = books();
    let fuse = Fuse::default();

    let indexed = fuse.search_text_in_fuse_list("man", &books);
    let borrowed = fuse.search_text_in_fuseable_refs("man", &books);

    assert_eq!(indexed.len(), borrowed.len());
    for (by_index, by_ref) in indexed.iter().zip(borrowed.iter()) {
        assert_eq!(by_index.index, by_ref.index);
        assert_eq!(by_index.score, by_ref.score);
        assert_eq!(by_index.results, by_ref.results);
        // the borrowed result points at the very item the index refers to
        assert_eq!(&books[by_index.index], by_ref.item);
    }
}

#[test]
fn fuseable_refs_searches_a_hash_set() {
    use std::collections::HashSet;

    let set: HashSet<Book> = books().into_iter().collect();
    let fuse = Fuse::default();
    let results = fuse.search_text_in_fuseable_refs("man", &set);

    assert_eq!(results.len(), 3);
    assert_eq!(results[0].item.author, "P.D. Mans");
    // every result borrows an item that really is in the set
    assert!(results.iter().all(|result| set.contains(result.item)));
    // best match first
    assert!(results.windows(2).all(|w| w[0].score <= w[1].score));
}

#[test]
fn fuseable_refs_searches_hash_map_values() {
    use std::collections::HashMap;

    let map: HashMap<usize, Book> = books().into_iter().enumerate().collect();
    let fuse = Fuse::default();
    let results = fuse.search_text_in_fuseable_refs("man", map.values());

    assert_eq!(results.len(), 3);
    assert_eq!(results[0].item.author, "P.D. Mans");
}

#[test]
fn fuseable_refs_handles_empty_input() {
    let fuse = Fuse::default();
    let empty: Vec<Book> = vec![];

    assert!(fuse.search_text_in_fuseable_refs("man", &empty).is_empty());
}

// Each `ranges` below really is a Vec holding one Range, which is what the lint asks about.
#[allow(clippy::single_range_in_vec_init)]
#[test]
fn scoring_is_stable() {
    use crate::{FResult, FuseableSearchResult};

    // Pins the absolute scores, so a change to the shared scoring core is caught
    // rather than being masked by the sequential/chunked equivalence tests.
    let books = [
        book("Old Man's War fiction", "John X"),
        book("Right Ho Jeeves", "P.D. Mans"),
    ];

    assert_eq!(
        Fuse::default().search_text_in_fuse_list("man", &books),
        vec![
            FuseableSearchResult {
                index: 1,
                score: 0.015000000000000003,
                results: vec![FResult {
                    value: String::from("author"),
                    score: 0.015000000000000003,
                    ranges: vec![5..8],
                }],
            },
            FuseableSearchResult {
                index: 0,
                score: 0.027999999999999997,
                results: vec![FResult {
                    value: String::from("title"),
                    score: 0.027999999999999997,
                    ranges: vec![4..7],
                }],
            },
        ]
    );
}

#[test]
fn exact_match_on_full_weight_property_scores_just_above_zero() {
    // A property of weight 1.0 that matches exactly scores 0.001 rather than 0.0,
    // so that it still sorts against other matches. The weights used by `books()`
    // never reach 1.0, so this case needs its own fixture.
    struct Single {
        name: String,
    }

    impl Fuseable for Single {
        fn properties(&self) -> Vec<FuseProperty> {
            vec![FuseProperty {
                value: String::from("name"),
                weight: 1.0,
            }]
        }

        fn lookup(&self, key: &str) -> Option<&str> {
            match key {
                "name" => Some(&self.name),
                _ => None,
            }
        }
    }

    let items = [Single {
        name: String::from("exact"),
    }];

    let results = Fuse::default().search_text_in_fuse_list("exact", &items);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].score, 0.001);
}

#[cfg(feature = "rayon")]
#[test]
fn chunked_rayon_matches_sequential() {
    use crate::FuseableSearchResult;

    let books = books();
    let fuse = Fuse::default();
    let expected = fuse.search_text_in_fuse_list("man", &books);

    for chunk_size in 1..=books.len() + 1 {
        fuse.search_text_in_fuse_list_with_chunk_size_rayon(
            "man",
            &books,
            chunk_size,
            &|results: Vec<FuseableSearchResult>| {
                assert_eq!(
                    results, expected,
                    "chunk size {chunk_size} disagreed with the sequential search"
                );
            },
        );
    }
}

#[cfg(feature = "rayon")]
#[test]
fn chunked_rayon_reports_global_indices() {
    use crate::FuseableSearchResult;

    // Regression: indices used to restart at 0 in every chunk, so with a chunk
    // size of 1 every result came back claiming index 0.
    let books = books();
    let fuse = Fuse::default();

    let check = |results: Vec<FuseableSearchResult>| {
        let mut indices: Vec<usize> = results.iter().map(|r| r.index).collect();
        indices.sort_unstable();
        assert_eq!(indices, vec![0, 1, 2]);
    };

    fuse.search_text_in_fuse_list_with_chunk_size_rayon("man", &books, 1, &check);
}

#[cfg(feature = "rayon")]
#[test]
fn fuseable_refs_rayon_matches_sequential() {
    use crate::FuseableRefSearchResult;

    let books = books();
    let fuse = Fuse::default();
    let expected = fuse.search_text_in_fuseable_refs("man", &books);

    for chunk_size in 0..=books.len() {
        fuse.search_text_in_fuseable_refs_with_chunk_size_rayon(
            "man",
            &books,
            chunk_size,
            &|results: Vec<FuseableRefSearchResult<Book>>| {
                assert_eq!(
                    results, expected,
                    "chunk size {chunk_size} disagreed with the sequential search"
                );
            },
        );
    }
}

#[cfg(feature = "async")]
#[test]
fn chunked_async_matches_sequential() {
    use crate::FuseableSearchResult;

    let books = books();
    let fuse = Fuse::default();
    let expected = fuse.search_text_in_fuse_list("man", &books);

    for chunk_size in 1..=books.len() + 1 {
        fuse.search_text_in_fuse_list_with_chunk_size(
            "man",
            &books,
            chunk_size,
            &|results: Vec<FuseableSearchResult>| {
                assert_eq!(
                    results, expected,
                    "chunk size {chunk_size} disagreed with the sequential search"
                );
            },
        );
    }
}

#[cfg(feature = "async")]
#[test]
fn chunked_async_reports_global_indices() {
    use crate::FuseableSearchResult;

    // Regression: see chunked_rayon_reports_global_indices.
    let books = books();
    let fuse = Fuse::default();

    let check = |results: Vec<FuseableSearchResult>| {
        let mut indices: Vec<usize> = results.iter().map(|r| r.index).collect();
        indices.sort_unstable();
        assert_eq!(indices, vec![0, 1, 2]);
    };

    fuse.search_text_in_fuse_list_with_chunk_size("man", &books, 1, &check);
}

#[cfg(feature = "async")]
#[test]
fn fuseable_refs_async_matches_sequential() {
    use crate::FuseableRefSearchResult;

    let books = books();
    let fuse = Fuse::default();
    let expected = fuse.search_text_in_fuseable_refs("man", &books);

    for chunk_size in 0..=books.len() {
        fuse.search_text_in_fuseable_refs_with_chunk_size(
            "man",
            &books,
            chunk_size,
            &|results: Vec<FuseableRefSearchResult<Book>>| {
                assert_eq!(
                    results, expected,
                    "chunk size {chunk_size} disagreed with the sequential search"
                );
            },
        );
    }
}

#[cfg(feature = "async")]
#[test]
fn fuseable_refs_async_handles_empty_input() {
    use crate::FuseableRefSearchResult;

    let fuse = Fuse::default();
    let empty: Vec<Book> = vec![];

    fuse.search_text_in_fuseable_refs_with_chunk_size("man", &empty, 2, &|results: Vec<
        FuseableRefSearchResult<Book>,
    >| {
        assert!(results.is_empty());
    });
}

#[cfg(feature = "derive")]
mod derive {
    use crate::{FuseProperty, Fuseable};

    #[derive(Fuseable)]
    struct Derived {
        #[fuse(weight = 0.3)]
        title: String,
        #[fuse(weight = 0.7)]
        author: String,
        #[allow(dead_code)]
        isbn: u64,
    }

    #[derive(Fuseable)]
    struct DefaultWeight {
        #[fuse]
        name: String,
    }

    #[derive(Fuseable)]
    struct Borrowed<'a> {
        #[fuse]
        text: &'a str,
    }

    fn derived() -> Derived {
        Derived {
            title: String::from("Old Man's War fiction"),
            author: String::from("John X"),
            isbn: 12345,
        }
    }

    #[test]
    fn generates_the_expected_properties() {
        assert_eq!(
            derived().properties(),
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
        );
    }

    #[test]
    fn lookup_resolves_marked_fields() {
        let item = derived();

        assert_eq!(item.lookup("title"), Some("Old Man's War fiction"));
        assert_eq!(item.lookup("author"), Some("John X"));
    }

    #[test]
    fn unmarked_fields_are_not_exposed() {
        let item = derived();

        // `isbn` carries no `#[fuse]`, so it is neither a property nor looked up.
        assert!(item.properties().iter().all(|p| p.value != "isbn"));
        assert_eq!(item.lookup("isbn"), None);
        assert_eq!(item.lookup("nonexistent"), None);
    }

    #[test]
    fn bare_attribute_defaults_to_full_weight() {
        let item = DefaultWeight {
            name: String::from("fiction"),
        };

        assert_eq!(
            item.properties(),
            vec![FuseProperty {
                value: String::from("name"),
                weight: 1.0,
            }]
        );
    }

    #[test]
    fn supports_borrowed_fields_and_lifetimes() {
        let item = Borrowed {
            text: "Not all those who wander are lost",
        };

        assert_eq!(
            item.lookup("text"),
            Some("Not all those who wander are lost")
        );
    }

    #[test]
    fn scores_identically_to_a_handwritten_impl() {
        use crate::Fuse;

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

        let fuse = Fuse::default();
        let derived_list = [derived()];
        let manual_list = [Manual {
            title: String::from("Old Man's War fiction"),
            author: String::from("John X"),
        }];

        assert_eq!(
            fuse.search_text_in_fuse_list("man", &derived_list),
            fuse.search_text_in_fuse_list("man", &manual_list)
        );
    }
}
