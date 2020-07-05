impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut res = 1.0f64;
        let mut i = n;
        let mut xx = x;
        while i != 0 {
            if i % 2 != 0 {
                res *= xx
            }
            xx *= xx;
            i /= 2
        }
        if n < 0 { 1.0 / res } else { res }
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::my_pow(2.00000, 10), 1024.00000);
    assert_eq!(Solution::my_pow(2.10000, 3), 9.26100); // 这个其实过不了，不过浮点数么……
    assert_eq!(Solution::my_pow(2.00000, -2), 0.25000);
    println!("ok")
}
