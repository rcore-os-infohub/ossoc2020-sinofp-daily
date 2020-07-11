impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let (mut l, mut r) = (0, nums.len() - 1);
        let mut i = 0;
        while i <= r {
            match nums[i] {
                0 => {
                    nums.swap(i, l);
                    l += 1;
                    i += 1;
                }
                2 => {
                    nums.swap(i, r);
                    if r > 0 { r -= 1; } else { return; }
                }
                _ => i += 1
            }
        }
    }
}

struct Solution;

fn main() {
    let mut v = vec![2, 0, 2, 1, 1, 0];
    Solution::sort_colors(&mut v);
    assert_eq!(v, vec![0, 0, 1, 1, 2, 2]);

    v = vec![2, 0, 1];
    Solution::sort_colors(&mut v);
    assert_eq!(v, vec![0, 1, 2]);

    v = vec![2];
    Solution::sort_colors(&mut v);
    assert_eq!(v, vec![2]);

    v = vec![2, 1, 2];
    Solution::sort_colors(&mut v);
    assert_eq!(v, vec![1, 2, 2]);

    v = vec![2, 2];
    Solution::sort_colors(&mut v);
    assert_eq!(v, vec![2, 2]);

    v = vec![0, 0];
    Solution::sort_colors(&mut v);
    assert_eq!(v, vec![0, 0]);

    println!("ok")
}
