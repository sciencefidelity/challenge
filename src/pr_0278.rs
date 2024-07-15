pub struct Solution {
    bad_version: i32,
}

impl Solution {
    pub const fn first_bad_version(&self, n: i32) -> i32 {
        let mut start = 0;
        let mut end = n;
        while end - start > 1 {
            let mid = start + (end - start) / 2;
            if self.is_bad_version(mid) {
                end = mid;
            } else {
                start = mid;
            }
        }
        end
    }

    pub const fn new(bad_version: i32) -> Self {
        Self { bad_version }
    }

    const fn is_bad_version(&self, version: i32) -> bool {
        version >= self.bad_version
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let product = Solution::new(4);
        assert_eq!(product.first_bad_version(5), 4);
    }

    #[test]
    fn case_2() {
        let product = Solution::new(1);
        assert_eq!(product.first_bad_version(1), 1);
    }

    #[test]
    fn case_3() {
        let product = Solution::new(4);
        assert_eq!(product.first_bad_version(10), 4);
    }
}
