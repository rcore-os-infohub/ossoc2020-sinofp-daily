impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        fn dfs(mut nums: Vec<i32>, idx: usize, res: &mut Vec<Vec<i32>>) {
            if idx == nums.len() {
                res.push(nums);
            } else {
                for i in idx..nums.len() {
                    nums.swap(i, idx);
                    dfs(nums.clone(), idx + 1, res);
                    nums.swap(idx, i);
                }
            }
        }
        dfs(nums, 0, &mut res);
        res
    }
}

struct Solution;

fn main() {
    println!("{:?}", Solution::permute(vec![1, 2, 3]));
    println!("ok")
}
