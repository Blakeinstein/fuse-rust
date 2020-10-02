use fuse_rust::{ Fuse, Fuseable, FuseProperty, FResult, FuseableSearchResult };

#[derive(Debug)]
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
        FuseableSearchResult{
            index: 1,
            score: 0.015000000000000003,
            results: vec!(FResult{
                value: String::from("author"),
                score: 0.015000000000000003,
                ranges: vec!((5..8)),
            }),
        },
        FuseableSearchResult{
            index: 0,
            score: 0.027999999999999997,
            results: vec!(FResult{
                value: String::from("title"),
                score: 0.027999999999999997,
                ranges: vec!((4..7)),
            })
        }
    ), "Fuseable Search returned incorrect results");
    
    results.iter().for_each(|result| 
        println!(r#"
index: {}
score: {}
results: {:?}
---------------"#, result.index, result.score, result.results)
    );
}