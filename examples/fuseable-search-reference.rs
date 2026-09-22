use fuse_rust::{Fuse, FuseProperty, Fuseable};
use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, Hash, Debug)]
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
        title: title.into(),
        author: author.into(),
    }
}

fn main() {
    let fuse = Fuse::default();

    // --- case 1: a HashSet ---
    let mut set: HashSet<Book> = HashSet::new();
    set.insert(book("Old Man's War fiction", "John X"));
    set.insert(book("Right Ho Jeeves", "P.D. Mans"));
    set.insert(book("The Silmarillion", "J.R.R. Tolkien"));

    // Borrow, don't clone: this Vec holds pointers, not Books.
    let borrowed: Vec<&Book> = set.iter().collect();
    let results = fuse.search_text_in_fuse_list("man", &borrowed);

    println!("HashSet matches for \"man\":");
    for r in &results {
        println!("  {:.4}  {}", r.score, borrowed[r.index].title);
    }

    // --- case 2: a HashMap ---
    let mut map: HashMap<u32, Book> = HashMap::new();
    map.insert(1, book("Old Man's War fiction", "John X"));
    map.insert(2, book("Right Ho Jeeves", "P.D. Mans"));

    // unzip keeps keys aligned with values, so r.index maps back to a key.
    let (keys, values): (Vec<&u32>, Vec<&Book>) = map.iter().unzip();
    let results = fuse.search_text_in_fuse_list("man", &values);

    println!("\nHashMap matches for \"man\":");
    for r in &results {
        println!(
            "  {:.4}  key={}  {}",
            r.score, keys[r.index], values[r.index].title
        );
    }
}
