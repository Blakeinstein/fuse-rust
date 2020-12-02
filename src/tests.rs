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

    Fuse::default().search_text_in_iterable(pat, s.iter());
}
