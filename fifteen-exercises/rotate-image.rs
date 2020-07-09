impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        fn swap(matrix: &mut Vec<Vec<i32>>, i1: usize, j1: usize, i2: usize, j2: usize) {
            let temp = matrix[i1][j1];
            matrix[i1][j1] = matrix[i2][j2];
            matrix[i2][j2] = temp;
        }
        let n = matrix.len();
        for i in 0..n {
            for j in 0..i + 1 {
                swap(matrix, i, j, j, i);
            }
        }
        for i in 0..n {
            for j in 0..n / 2 {
                swap(matrix, i, j, i, n - 1 - j);
            }
        }
    }
}

struct Solution;

fn main() {
    let m = &mut vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    Solution::rotate(m);
    assert_eq!(m, &mut vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);
    println!("ok")
}
