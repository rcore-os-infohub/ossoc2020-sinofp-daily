impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        if matrix.len() == 0 { return Vec::new() }
        let (x_max, y_max) = (matrix[0].len(), matrix.len());
        let mut vis = vec![vec![false; x_max]; y_max]; // 也可以直接用可变matrix记录
        let (mut x, mut y) = (-1, 0);
        let dx = vec![1i32, 0, -1, 0];
        let dy = vec![0i32, 1, 0, -1];
        let mut dir = 0usize;
        let mut res = Vec::new();
        loop {
            loop {
                x = x + dx[dir];
                y = y + dy[dir];
                if x < 0 || x >= x_max as i32 || y < 0 || y >= y_max as i32 || vis[y as usize][x as usize] {
                    x -= dx[dir];
                    y -= dy[dir]; // 退回去
                    break;
                }
                res.push(matrix[y as usize][x as usize]);
                vis[y as usize][x as usize] = true;
            }
            if res.len() == x_max * y_max {
                break;
            }
            dir = (dir + 1) % 4;
        }
        res
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]));
    println!("ok")
}
