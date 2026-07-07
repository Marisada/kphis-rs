// from https://gitlab.com/EnricoCh/rust-fuzzy-search/-/blob/main/src/lib.rs
use std::iter;

type Trigram = Vec<(char, char, char)>;

pub fn trigrams(s: &str) -> Trigram {
    let s = s.to_ascii_lowercase();
    let it_1 = iter::once(' ').chain(iter::once(' ')).chain(s.chars());
    let it_2 = iter::once(' ').chain(s.chars());
    let it_3 = s.chars().chain(iter::once(' '));

    let res: Vec<(char, char, char)> = it_1.zip(it_2).zip(it_3).map(|((a, b), c): ((char, char), char)| (a, b, c)).collect();
    res
}

pub fn fuzzy_compare(trigrams_a: &Trigram, chars_count: usize, b: &str) -> f32 {
    // gets the trigrams for both strings
    let trigrams_b = trigrams(b);

    // accumulator
    let mut acc: f32 = 0.0;
    // counts the number of trigrams of the first string that are also present in the second one
    for t_a in trigrams_a {
        for t_b in &trigrams_b {
            if t_a == t_b {
                acc += 1.0;
                break;
            }
        }
    }
    let res = acc / (chars_count as f32);
    // crops between zero and one
    match res {
        ..0.0 => 0.0,
        1.0.. => 1.0,
        res => res,
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_trigrams() {
        let s = "abcde";
        let trig = trigrams(s);
        let expect = vec![(' ', ' ', 'a'), (' ', 'a', 'b'), ('a', 'b', 'c'), ('b', 'c', 'd'), ('c', 'd', 'e'), ('d', 'e', ' ')];
        assert_eq!(trig, expect);
    }

    #[test]
    pub fn test_fuzzy_compare_first() {
        // (' ', ' ', 'a'), <= match
        // (' ', 'a', 'b'), <= match
        // ('a', 'b', 'c'), <= match
        // ('b', 'c', ' '),
        let trigrams_a = trigrams("abc");

        // (' ', ' ', 'a'), <= match
        // (' ', 'a', 'b'), <= match
        // ('a', 'b', 'c'), <= match
        // ('b', 'c', 'd'),
        // ('c', 'd', 'e'),
        // ('d', 'e', ' '),
        let res = fuzzy_compare(&trigrams_a, 3, "abcde");
        assert_eq!(res, 3.0 / 3.0);
    }

    #[test]
    pub fn test_fuzzy_compare_mid() {
        // (' ', ' ', 'b'),
        // (' ', 'b', 'c'),
        // ('b', 'c', 'd'), <= match
        // ('c', 'd', ' '),
        let trigrams_a = trigrams("bcd");

        // (' ', ' ', 'a'),
        // (' ', 'a', 'b'),
        // ('a', 'b', 'c'),
        // ('b', 'c', 'd'), <= match
        // ('c', 'd', 'e'),
        // ('d', 'e', ' '),
        let res = fuzzy_compare(&trigrams_a, 3, "abcde");
        assert_eq!(res, 1.0 / 3.0);
    }

    #[test]
    pub fn test_fuzzy_compare_end() {
        // (' ', ' ', 'c'),
        // (' ', 'c', 'd'),
        // ('c', 'd', 'e'), <= match
        // ('d', 'e', ' '), <= match
        let trigrams_a = trigrams("cde");

        // (' ', ' ', 'a'),
        // (' ', 'a', 'b'),
        // ('a', 'b', 'c'),
        // ('b', 'c', 'd'),
        // ('c', 'd', 'e'), <= match
        // ('d', 'e', ' '), <= match
        let res = fuzzy_compare(&trigrams_a, 3, "abcde");
        assert_eq!(res, 2.0 / 3.0);
    }

    #[test]
    pub fn test_fuzzy_compare_multiple() {
        // (' ', ' ', 'a'), <= match
        // (' ', 'a', 'b'), <= match
        // ('a', 'b', 'c'), <= match
        // ('b', 'c', ' '), <= match
        let trigrams_a = trigrams("abc");

        // (' ', ' ', 'a'), <= match
        // (' ', 'a', 'b'), <= match
        // ('a', 'b', 'c'), <= match
        // ('b', 'c', 'a'),
        // ('c', 'a', 'b'),
        // ('a', 'b', 'c'), <= match
        // ('b', 'c', ' '), <= match
        let res = fuzzy_compare(&trigrams_a, 3, "abcabc");
        // 5.0 / 3.0 will crop to 1.0
        assert_eq!(res, 1.0);
    }
}
