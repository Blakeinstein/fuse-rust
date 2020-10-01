use fuse_rs::{ Fuse, SearchResult};

fn test(list: Vec<SearchResult>) {
    dbg!(list);
}
fn main() {
    let fuse = Fuse::default();
    let random_strings = [
        "tbtlaafazm",
        "koyqdadlgq",
        "oimiuidxph",
        "vpsduaanow",
        "hebiahfitj",
        "npwhrthmil",
        "azrwbimxwv",
        "vcsawdweuu",
        "rxkratrkmy",
        "aylajveblo",
        "rrxcujnscn",
        "qiquwmbjnq",
        "rnhfquhitv",
        "fdaerpicep",
        "uqdxisyife",
        "cjjoczaokp",
        "rzyqcbsysx",
        "hbbpyleeld",
        "agcpswynrh",
        "yfszgoorut",
        "bgqyspeffj",
        "izbqqtbicy",
        "fmtylhheez",
        "qwqmrjgsof",
        "ukxctnwjoa",
        "dudeqiiywj",
        "tkzoipxcwj",
        "ksceoqifgh",
        "ibganykxkk",
        "xjcmlkipmx",
        "lqlymvienh",
        "oixeifwozn",
        "rcliwhskci",
        "egccrviiht",
        "phfyrggvns",
        "wyppwykhlr",
        "jzbdxsvtnh",
        "zliedzschj",
        "hqvvdzmosr",
        "xkmcraghkf",
        "blyvvzlfvn",
        "snyozhntqh",
        "evlondyrqy",
        "sixjfceouu",
        "jtkryuwqug",
        "qceukaadkw",
        "sadaexvhps",
        "ikxraiifbo",
        "ilopqywxxd",
        "dcabhorpap",
        "kkvzxmqjmk",
        "euzucvvxrt",
        "cgqllottas",
        "ziyhnyjwly",
        "iczgeymhsz",
        "vqeccwggup",
        "srjpkgjsrm",
        "tdvlcuabbh",
        "egkbeovaet",
        "oqjnttcbnj",
        "mlspwbbbjl",
        "bbxyujiptq",
        "exjrsuxblo",
        "zoadfaqwqo",
        "dlqryyrnqr",
        "sdxdddshpu",
        "cszwyvgnbb",
        "ejyzvbhecn",
        "twacwdcmvp",
        "gtfljpvnuw",
        "bgjorqekho",
        "rqlgsjdcyn",
        "ssglbheril",
        "jaizdkhsuw",
        "cdbvmuicqu",
        "pesefozfyj",
        "tkjazdchyc",
        "enafivjxst",
        "nbvninlnqf",
        "aflmujlxxy",
        "edydtfyvpr",
        "nrrdxpibno",
        "lzavdwnasq",
        "cfmkkspgxd",
        "zkieyyejli",
        "uzgjmycefq",
        "pwhuafsdmd",
        "htblqmlnem",
        "loqhdoggub",
        "thaneejlnz",
        "tffydqyvuv",
        "ubatmpwlvq",
        "lqmspwfeuh",
        "upfvzbshvs",
        "uezupxkawl",
        "nlttwsdgcq",
        "vqjbleazcs",
        "faobxtvixg",
        "gbtqzvyiqf",
        "itovolfijo",
    ];
    
    fuse.search_text_in_string_list(
        "aa",
        &random_strings,
        10 as usize,
        &test,
    );
}