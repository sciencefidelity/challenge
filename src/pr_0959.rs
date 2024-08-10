pub struct Solution;

// use std::collections::VecDeque;
//
// const DIRECTIONS: &[(i32, i32); 4] = &[(0, 1), (0, -1), (1, 0), (-1, 0)];
//
// impl Solution {
//     pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
//         let grid_size = grid.len();
//         let mut expanded_grid = vec![vec![0; grid_size * 3]; grid_size * 3];
//         for (i, row) in grid.into_iter().enumerate() {
//             for (j, col) in row.chars().enumerate() {
//                 let (base_row, base_col) = (i * 3, j * 3);
//                 if col == '\\' {
//                     expanded_grid[base_row][base_col] = 1;
//                     expanded_grid[base_row + 1][base_col + 1] = 1;
//                     expanded_grid[base_row + 2][base_col + 2] = 1;
//                 } else if col == '/' {
//                     expanded_grid[base_row][base_col + 2] = 1;
//                     expanded_grid[base_row + 1][base_col + 1] = 1;
//                     expanded_grid[base_row + 2][base_col] = 1;
//                 }
//             }
//         }
//         let mut region_count = 0;
//         for i in 0..grid_size * 3 {
//             for j in 0..grid_size * 3 {
//                 if expanded_grid[i][j] == 0 {
//                     Self::flood_fill(
//                         &mut expanded_grid,
//                         i32::try_from(i).unwrap(),
//                         i32::try_from(j).unwrap(),
//                     );
//                     region_count += 1;
//                 }
//             }
//         }
//         region_count
//     }
//
//     fn flood_fill(expanded_grid: &mut [Vec<i32>], row: i32, col: i32) {
//         let mut queue = VecDeque::new();
//         expanded_grid[usize::try_from(row).unwrap()][usize::try_from(col).unwrap()] = 1;
//         queue.push_back((row, col));
//         while let Some((current_row, current_col)) = queue.pop_front() {
//             for direction in DIRECTIONS {
//                 let new_row = direction.0 + current_row;
//                 let new_col = direction.1 + current_col;
//                 if Self::is_valid_cell(expanded_grid, new_row, new_col) {
//                     expanded_grid[usize::try_from(new_row).unwrap()]
//                         [usize::try_from(new_col).unwrap()] = 1;
//                     queue.push_back((new_row, new_col));
//                 }
//             }
//         }
//     }
//
//     fn is_valid_cell(expanded_grid: &[Vec<i32>], row: i32, col: i32) -> bool {
//         let n = i32::try_from(expanded_grid.len()).unwrap();
//         row >= 0
//             && col >= 0
//             && row < n
//             && col < n
//             && expanded_grid[usize::try_from(row).unwrap()][usize::try_from(col).unwrap()] == 0
//     }
// }

// impl Solution {
//     pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
//         let grid_size = i32::try_from(grid.len()).unwrap();
//         let total_triangles = grid_size * grid_size * 4;
//         let mut parent_array = vec![-1; usize::try_from(total_triangles).unwrap()];
//         let mut region_count = total_triangles;
//         for (i, row) in grid.into_iter().enumerate() {
//             let i = i32::try_from(i).unwrap();
//             for (j, col) in row.chars().enumerate() {
//                 let j = i32::try_from(j).unwrap();
//                 if i > 0 {
//                     region_count -= Self::union_triangles(
//                         &mut parent_array,
//                         Self::get_triangle_index(grid_size, i - 1, j, 2),
//                         Self::get_triangle_index(grid_size, i, j, 0),
//                     );
//                 }
//                 if j > 0 {
//                     region_count -= Self::union_triangles(
//                         &mut parent_array,
//                         Self::get_triangle_index(grid_size, i, j - 1, 1),
//                         Self::get_triangle_index(grid_size, i, j, 3),
//                     );
//                 }
//                 if col != '/' {
//                     region_count -= Self::union_triangles(
//                         &mut parent_array,
//                         Self::get_triangle_index(grid_size, i, j, 0),
//                         Self::get_triangle_index(grid_size, i, j, 1),
//                     );
//                     region_count -= Self::union_triangles(
//                         &mut parent_array,
//                         Self::get_triangle_index(grid_size, i, j, 2),
//                         Self::get_triangle_index(grid_size, i, j, 3),
//                     );
//                 }
//                 if col != '\\' {
//                     region_count -= Self::union_triangles(
//                         &mut parent_array,
//                         Self::get_triangle_index(grid_size, i, j, 0),
//                         Self::get_triangle_index(grid_size, i, j, 3),
//                     );
//                     region_count -= Self::union_triangles(
//                         &mut parent_array,
//                         Self::get_triangle_index(grid_size, i, j, 2),
//                         Self::get_triangle_index(grid_size, i, j, 1),
//                     );
//                 }
//             }
//         }
//         region_count
//     }
//
//     const fn get_triangle_index(grid_size: i32, row: i32, col: i32, triangle_num: i32) -> i32 {
//         (grid_size * row + col) * 4 + triangle_num
//     }
//
//     fn union_triangles(parent_array: &mut Vec<i32>, x: i32, y: i32) -> i32 {
//         let parent_x = Self::find_parent(parent_array, x);
//         let parent_y = Self::find_parent(parent_array, y);
//         if parent_x != parent_y {
//             parent_array[usize::try_from(parent_x).unwrap()] = parent_y;
//             return 1;
//         }
//         0
//     }
//
//     fn find_parent(parent_array: &mut Vec<i32>, x: i32) -> i32 {
//         if parent_array[usize::try_from(x).unwrap()] == -1 {
//             return x;
//         }
//         parent_array[usize::try_from(x).unwrap()] =
//             Self::find_parent(parent_array, parent_array[usize::try_from(x).unwrap()]);
//         parent_array[usize::try_from(x).unwrap()]
//     }
// }

impl Solution {
    pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
        let grid_size = i32::try_from(grid.len()).unwrap();
        let points_per_side = grid_size + 1;
        let total_points = points_per_side.pow(2);
        let mut parent_array = vec![-1; usize::try_from(total_points).unwrap()];
        for i in 0..points_per_side {
            for j in 0..points_per_side {
                if i == 0 || j == 0 || i == points_per_side - 1 || j == points_per_side - 1 {
                    let point = i * points_per_side + j;
                    parent_array[usize::try_from(point).unwrap()] = 0;
                }
            }
        }
        parent_array[0] = -1;
        let mut region_count = 1;
        for (i, row) in grid.into_iter().enumerate() {
            let i = i32::try_from(i).unwrap();
            for (j, col) in row.chars().enumerate() {
                let j = i32::try_from(j).unwrap();
                if col == '/' {
                    let top_right = i * points_per_side + (j + 1);
                    let bottom_left = (i + 1) * points_per_side + j;
                    region_count += Self::union_sets(&mut parent_array, top_right, bottom_left);
                } else if col == '\\' {
                    let top_left = i * points_per_side + j;
                    let bottom_right = (i + 1) * points_per_side + (j + 1);
                    region_count += Self::union_sets(&mut parent_array, top_left, bottom_right);
                }
            }
        }
        region_count
    }

    fn find(parent_array: &mut Vec<i32>, node: i32) -> i32 {
        if parent_array[usize::try_from(node).unwrap()] == -1 {
            return node;
        }
        parent_array[usize::try_from(node).unwrap()] =
            Self::find(parent_array, parent_array[usize::try_from(node).unwrap()]);
        parent_array[usize::try_from(node).unwrap()]
    }

    fn union_sets(parent_array: &mut Vec<i32>, node1: i32, node2: i32) -> i32 {
        let parent1 = Self::find(parent_array, node1);
        let parent2 = Self::find(parent_array, node2);
        if parent1 == parent2 {
            return 1;
        }
        parent_array[usize::try_from(parent2).unwrap()] = parent1;
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        assert_eq!(
            2,
            Solution::regions_by_slashes(vec![" /".to_owned(), "/ ".to_owned()])
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            1,
            Solution::regions_by_slashes(vec![" /".to_owned(), "  ".to_owned()])
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            5,
            Solution::regions_by_slashes(vec!["/\\".to_owned(), "\\/".to_owned()])
        );
    }
}

/*
[0, 0, 1, 1, 0, 0]
[0, 1, 0, 0, 1, 0]
[1, 0, 0, 0, 0, 1]
[1, 0, 0, 0, 0, 0]
[0, 1, 0, 0, 0, 0]
[0, 0, 1, 0, 0, 0]


*/
