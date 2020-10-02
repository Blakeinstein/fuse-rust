use fuse_rs::{ Fuse, ScoreResult};

fn main() {
    let fuse = Fuse::default();
    let text = "Old Man's War";
    let search_text = "od mn war";

    let result = fuse.search_text_in_string(search_text, text);
    assert_eq!(result, Some(ScoreResult{
        score: 0.4444444444444444,
        ranges: vec!((0..1), (2..7), (9..13)),
    }), "Simple search returned incorrect results");
    dbg!(result);
}
