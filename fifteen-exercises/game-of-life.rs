impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let last_board = board.clone();
        let dx = vec![-1, 0, 1, -1, 1, -1, 0, 1];
        let dy = vec![-1, -1, -1, 0, 0, 1, 1, 1];
        let x_max = board[0].len();
        let y_max = board.len();
        for y in 0..y_max {
            for x in 0..x_max {
                let mut living_neighbours = 0;
                for i in 0usize..8 {
                    let nx = x as i32 + dx[i];
                    let ny = y as i32 + dy[i];
                    if nx > -1 && nx < x_max as i32 && ny > -1 && ny < y_max as i32 {
                        living_neighbours += last_board[ny as usize][nx as usize];
                    }
                }
                if last_board[y][x] == 1 {
                    board[y][x] = if living_neighbours == 2 || living_neighbours == 3 { 1 } else { 0 };
                } else {
                    board[y][x] = if living_neighbours == 3 { 1 } else { 0 };
                }
            }
        }
    }
}

struct Solution;

fn main() {
    let board: &mut Vec<Vec<i32>> = &mut vec![vec![0i32, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];
    Solution::game_of_life(board);
    assert_eq!(board, &mut vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 1, 0]]);
    println!("ok")
}
