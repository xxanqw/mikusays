// Art supplied in https://github.com/xxanqw/mikusays/issues/25
// Original source: https://emojicombos.com/teto-ascii-art
use rand::RngExt;

const STYLES: [&str; 20] = [
    include_str!("teto/small.txt"),
    include_str!("teto/large.txt"),
    include_str!("teto/style-02.txt"),
    include_str!("teto/style-03.txt"),
    include_str!("teto/style-04.txt"),
    include_str!("teto/style-05.txt"),
    include_str!("teto/style-06.txt"),
    include_str!("teto/style-07.txt"),
    include_str!("teto/style-08.txt"),
    include_str!("teto/style-09.txt"),
    include_str!("teto/style-10.txt"),
    include_str!("teto/style-11.txt"),
    include_str!("teto/style-12.txt"),
    include_str!("teto/style-13.txt"),
    include_str!("teto/style-14.txt"),
    include_str!("teto/style-15.txt"),
    include_str!("teto/style-16.txt"),
    include_str!("teto/style-17.txt"),
    include_str!("teto/style-18.txt"),
    include_str!("teto/style-19.txt"),
];

pub fn get_teto_art(style: Option<i32>, list: Option<bool>) -> Vec<&'static str> {
    if list.unwrap_or(false) {
        return vec![" "; STYLES.len()];
    }

    let style = style.unwrap_or(-1);
    let index = if style < 0 {
        rand::rng().random_range(0..STYLES.len())
    } else {
        style as usize
    };
    STYLES[index].lines().collect()
}
