use fuse_rs::Fuse;

fn main() {
    let fuse = Fuse::default();
    let books = [
        "The Silmarillion",
        "The Lock Artist",
        "The Lost Symbol"
    ];
    let search_pattern = fuse.create_pattern("Te silm");

    books.iter().for_each(|&item| {
        let result = fuse.search(search_pattern.as_ref(), &item);
        dbg!(result);
    });
    // let results = fuse.search_text_in_iterable("Te silm", books.iter());
    // dbg!(results);
}
