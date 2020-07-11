impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let nums: Vec<usize> = nums.iter().map(|&n| n as usize).collect();
        let len = nums.len();
        let mut can_jump = vec![false; len];
        can_jump[0] = true;
        for i in 0..len - 1 {
            if can_jump[i] {
                for j in 1..std::cmp::min(len, i + 1 + nums[i]) {
                    can_jump[j] = true;
                }
            }
        }
        can_jump[len - 1]
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 4]), true);
    assert_eq!(Solution::can_jump(vec![3, 2, 1, 0, 4]), false);
    assert_eq!(Solution::can_jump(vec![0, 2, 3]), false);
    assert_eq!(Solution::can_jump(vec![1]), true);
    println!("ok")
}
