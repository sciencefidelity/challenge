pub trait Array {
    type Item;
    fn process(&self) -> Self::Item;
}

impl Array for &str {
    type Item = String;
    fn process(&self) -> Self::Item {
        (*self).to_string()
    }
}

impl Array for i32 {
    type Item = Self;
    fn process(&self) -> Self::Item {
        *self
    }
}

#[macro_export]
macro_rules! arr {
    [$([$($d:expr),*]),*] => {
        vec![
            $(
                vec![$($crate::Array::process(&$d)),*],
            )*
        ]
    };
    [$($d:expr),*] => {
        vec![$($crate::Array::process(&$d)),*]
    };
}

#[cfg(test)]
mod tests {
    use super::arr;

    #[test]
    fn test_array_of_strs() {
        let input = vec!["hello".to_owned(), "world".to_owned()];
        let expected = arr!["hello", "world"];
        assert_eq!(expected, input);
    }

    #[test]
    fn test_array_of_array_of_ints() {
        let expected = vec![vec![1, 2, 3], vec![4, 5, 6]];
        assert_eq!(expected, arr![[1, 2, 3], [4, 5, 6]]);
    }

    #[test]
    fn test_array_of_array_of_ints_of_different_lengths() {
        let expected = vec![vec![1, 2, 3], vec![4, 5]];
        assert_eq!(expected, arr![[1, 2, 3], [4, 5]]);
    }

    #[test]
    fn test_array_of_array_of_strs() {
        let vec = vec![
            vec!["a".to_owned(), "b".to_owned(), "c".to_owned()],
            vec!["d".to_owned(), "e".to_owned(), "f".to_owned()],
        ];
        assert_eq!(vec, arr![["a", "b", "c"], ["d", "e", "f"]]);
    }

    #[test]
    fn test_array_of_array_of_strs_of_different_lengths() {
        let vec = vec![
            vec!["a".to_owned(), "b".to_owned(), "c".to_owned()],
            vec!["d".to_owned(), "e".to_owned()],
        ];
        assert_eq!(vec, arr![["a", "b", "c"], ["d", "e"]]);
    }
}
