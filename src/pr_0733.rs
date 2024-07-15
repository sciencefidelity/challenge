pub struct Solution;

impl Solution {
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let (sr, sc): (usize, usize) = (sr.try_into().unwrap(), sc.try_into().unwrap());
        let old_color = image[sr][sc];
        if old_color != color {
            Self::dfs(&mut image, sr, sc, old_color, color);
        }
        image
    }

    fn dfs(img: &mut Vec<Vec<i32>>, sr: usize, sc: usize, old_color: i32, new_color: i32) {
        if img[sr][sc] == old_color {
            img[sr][sc] = new_color;
            if sr > 0 {
                Self::dfs(img, sr - 1, sc, old_color, new_color);
            }
            if sr + 1 < img.len() {
                Self::dfs(img, sr + 1, sc, old_color, new_color);
            }
            if sc > 0 {
                Self::dfs(img, sr, sc - 1, old_color, new_color);
            }
            if sc + 1 < img[0].len() {
                Self::dfs(img, sr, sc + 1, old_color, new_color);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::flood_fill(vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]], 1, 1, 2),
            vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]]
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::flood_fill(vec![vec![0, 0, 0], vec![0, 0, 0]], 0, 0, 0),
            vec![vec![0, 0, 0], vec![0, 0, 0]]
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::flood_fill(vec![vec![0, 0, 0], vec![0, 0, 0]], 1, 0, 2),
            vec![vec![2, 2, 2], vec![2, 2, 2]]
        );
    }
}
