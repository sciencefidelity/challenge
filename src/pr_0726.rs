pub struct Solution;

use std::collections::BTreeMap;

impl Solution {
    pub fn count_of_atoms(formula: String) -> String {
        let mut muls = vec![0; formula.len()];
        let mut stack = Vec::with_capacity(formula.len());
        stack.push(1);
        let (mut curr_number, mut curr_mul, mut running_mul) = (0, 1, 1);
        for (i, c) in formula.chars().rev().enumerate() {
            if c.is_numeric() {
                curr_number +=
                    TryInto::<i32>::try_into(c.to_digit(10).unwrap()).unwrap() * curr_mul;
                curr_mul *= 10;
            } else if c.is_alphabetic() {
                (curr_number, curr_mul) = (0, 1);
            } else if c == ')' {
                let curr_multiplier = if curr_number > 0 { curr_number } else { 1 };
                running_mul *= curr_multiplier;
                stack.push(curr_multiplier);
                (curr_number, curr_mul) = (0, 1);
            } else if c == '(' {
                running_mul /= stack.pop().unwrap();
                (curr_number, curr_mul) = (0, 1);
            }
            muls[formula.len() - 1 - i] = running_mul;
        }
        println!("{}", running_mul);
        let mut final_map: BTreeMap<String, i32> = BTreeMap::new();
        let mut formula_iter = formula.chars().enumerate().peekable();
        while let Some((i, c)) = formula_iter.next() {
            if c.is_ascii_uppercase() {
                let mut curr_atom = String::new();
                curr_atom.push(c);
                let mut curr_count = String::new();
                while formula_iter
                    .peek()
                    .is_some_and(|(_, c)| c.is_ascii_lowercase())
                {
                    curr_atom.push(formula_iter.peek().unwrap().1);
                    formula_iter.next();
                }
                while formula_iter.peek().is_some_and(|(_, c)| c.is_numeric()) {
                    curr_count.push(formula_iter.peek().unwrap().1);
                    formula_iter.next();
                }
                let count = if curr_count.len() > 0 {
                    curr_count.parse().unwrap()
                } else {
                    1
                };
                *final_map.entry(curr_atom).or_insert(0) += count * muls[i];
            }
        }
        println!("{muls:?}");
        let mut answer = String::new();
        let mut final_map_iter = final_map.iter();
        while let Some((key, val)) = final_map_iter.next() {
            let atom = if *val > 1 {
                format!("{key}{val}")
            } else {
                format!("{key}")
            };
            answer.push_str(atom.as_str());
        }
        answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(Solution::count_of_atoms("H2O".to_owned()), "H2O".to_owned());
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::count_of_atoms("Mg(OH)2".to_owned()),
            "H2MgO2".to_owned()
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::count_of_atoms("K4(ON(SO3)2)2".to_owned()),
            "K4N2O14S4".to_owned()
        );
    }

    #[test]
    fn case_4() {
        assert_eq!(
            Solution::count_of_atoms("(B2O39He17BeBe49)39".to_owned()),
            "B78Be1950He663O1521".to_owned()
        );
    }
}
