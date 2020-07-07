use std::fs::read;

impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        if citations.len() == 0 { return 0 }

        let mut l = 0usize;
        let mut r = citations.len() - 1;
        let mut h = 0;
        while l <= r { // vec![1] 也得进来
            let mid = (r + l) / 2;
            let cnt = (citations.len() - mid) as i32;
            if citations[mid] >= cnt {
                h = cnt;
                if mid == 0 {
                    break;
                }
                r = mid - 1
            } else {
                l = mid + 1
            }
        }
        h
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::h_index(vec![0, 1, 3, 5, 6]), 3);
    assert_eq!(Solution::h_index(vec![6, 6, 6, 6, 6, 6]), 6);
    assert_eq!(Solution::h_index(vec![0]), 0);
    assert_eq!(Solution::h_index(vec![1]), 1);
    println!("ok")
}
