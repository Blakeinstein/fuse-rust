use fuse_rs::{ Fuse, Fuseable, FuseProperty };

#[derive(Debug)]
struct Book<'a> {
    title: &'a str,
    author: &'a str,
}

impl Fuseable for Book<'_>{
    fn properties() -> Vec<FuseProperty> {
        return vec!(
            FuseProperty{name: String::from("title"), weight: 0.3},
            FuseProperty{name: String::from("author"), weight: 0.7},
        )
    }
}
fn main() {    
    let books = [
        Book{author: "John X", title: "Old Man's War fiction"},
        Book{author: "P.D. Mans", title: "Right Ho Jeeves"},
    ];
    
    let fuse = Fuse::default();
    // let results = fuse.search("man", in: books);
    
    // results.forEach { item in
    //     print("index: " + item.index)
    //     print("score: " + item.score)
    //     print("results: " + item.results)
    //     print("---------------")
    // }
    dbg!(books);
}