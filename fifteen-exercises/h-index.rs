impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut citations = citations;
        citations.sort();
        citations.reverse();
        let mut h = 0i32;
        let mut cnt = 0;
        for citation in citations {
            cnt += 1;
            if citation >= cnt {
                h = cnt
            }
        }
        h
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::h_index(vec![3,0,6,1,5]), 3);
    assert_eq!(Solution::h_index(vec![0]), 0);
    assert_eq!(Solution::h_index(vec![1]), 1);
    println!("ok")
}
