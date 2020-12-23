use crate::Fuse;

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

    assert_eq!(
        &s[r.start..r.end],
        needle
    );
}