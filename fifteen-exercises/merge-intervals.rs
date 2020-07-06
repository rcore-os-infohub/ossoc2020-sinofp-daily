impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut intervals = intervals;
        intervals.sort();
        let mut new_intervals = Vec::new();
        let mut i = 0usize;
        while i < intervals.len() {
            let l = intervals[i][0];
            let mut r = intervals[i][1];
            let mut j = i + 1;
            while j < intervals.len() {
                if intervals[j][0] <= r {
                    r = std::cmp::max(intervals[j][1], r) // [[1,4],[2,3]] → [[1,4]]
                } else {
                    break
                }
                j += 1;
            }
            new_intervals.push(vec![l, r]);
            i = j;
        }
        new_intervals
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]), vec![vec![1, 6], vec![8, 10], vec![15, 18]]);
    assert_eq!(Solution::merge(vec![vec![1,4],vec![2,3]]), vec![vec![1,4]]);
    println!("ok")
}