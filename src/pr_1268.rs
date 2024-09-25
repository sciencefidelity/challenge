pub struct Solution;

impl Solution {
    #[allow(clippy::needless_pass_by_value)]
    pub fn suggested_products(_products: Vec<String>, _search_word: String) -> Vec<Vec<String>> {
        vec![]
    }
}

#[derive(Default)]
struct Trie {
    children: [Option<Box<Trie>>; 26],
    is_word: bool,
}

impl Trie {
    pub fn new() -> Self {
        Self::default()
    }

    // pub fn dfs_with_prefix(curr: )
}

#[cfg(test)]
mod tests {
    use super::*;
    use utils::arr;

    #[test]
    fn case_1() {
        let products = arr!["mobile", "mouse", "moneypot", "monitor", "mousepad"];
        let search_word = "mouse".to_owned();
        let expected = arr![
            ["mobile", "moneypot", "monitor"],
            ["mobile", "moneypot", "monitor"],
            ["mouse", "mousepad"],
            ["mouse", "mousepad"],
            ["mouse", "mousepad"]
        ];
        assert_eq!(
            Solution::suggested_products(products, search_word),
            expected
        );
    }

    #[test]
    fn case_2() {
        let products = arr!["havana"];
        let search_word = "havana".to_owned();
        let expected = arr![
            ["havana"],
            ["havana"],
            ["havana"],
            ["havana"],
            ["havana"],
            ["havana"]
        ];
        assert_eq!(
            Solution::suggested_products(products, search_word),
            expected
        );
    }
}
