// Art supplied in https://github.com/xxanqw/mikusays/issues/25
// Original source: https://emojicombos.com/teto-ascii-art
use rand::RngExt;

const STYLES: [&str; 2] = [
    include_str!("teto/small.txt"),
    include_str!("teto/large.txt"),
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
