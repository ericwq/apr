/// https://www.ditig.com/256-colors-cheat-sheet
/// https://en.wikipedia.org/wiki/ANSI_escape_code#Colors
///
#[derive(Debug, PartialEq, Eq)]
pub enum Color {
    Ansi(u8),
    Palette(u8),
    Rgb(u8, u8, u8),
}

impl Default for Color {
    fn default() -> Self {
        Self::Rgb(0, 0, 0)
    }
}
// use std::collections::HashMap;
// use std::sync::OnceLock;
//
// fn hashmap() -> &'static HashMap<i32, Color> {
//     static HASHMAP: OnceLock<HashMap<i32, Color>> = OnceLock::new();
//     HASHMAP.get_or_init(|| HashMap::from([(30, Color::Ansi(30)), (31, Color::Ansi(31))]))
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_of_val() {
        let test_case = vec![
            (Color::Ansi(30), 4),
            (Color::Palette(12), 4),
            (Color::Rgb(255, 255, 0), 4),
        ];
        for (color, bytes_num) in test_case {
            assert_eq!(std::mem::size_of_val(&color), bytes_num);
        }
    }

    #[test]
    fn default() {
        assert_eq!(Color::default(), Color::Rgb(0, 0, 0));
    }
}
