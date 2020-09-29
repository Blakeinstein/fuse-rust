// extern crate fuse_rs;
use fuse_rs::Fuse;

fn main() {
    let fuse = Fuse::default();
    let text = "Old Man's War";
    let search_text = "od mn war";

    let result = fuse.search_text_in_string(search_text, text);
    dbg!(result);
}
