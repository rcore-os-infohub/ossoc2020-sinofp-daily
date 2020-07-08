impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if 1 == num_rows { return s; }
        let len = s.len();
        let md = (2 * num_rows - 2) as usize;
        let mut s_out = String::new();
        let mut i = 0;
        while i < len {
            s_out.push(s[i..].chars().next().unwrap());
            i += md;
        }
        for j in 1..(md / 2) + 1 {
            i = 0;
            while i < len {
                let l = i + j;
                let r = i + md - j;
                if l < len {
                    s_out.push(s[l..].chars().next().unwrap());
                }
                if r < len && r != l { // 最下面一行数字r==l，不判断会输出两次
                    s_out.push(s[r..].chars().next().unwrap());
                }
                i += md;
            }
        }
        s_out
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::convert("LEETCODEISHIRING".into(), 3), "LCIRETOESIIGEDHN".to_owned());
    assert_eq!(Solution::convert("LEETCODEISHIRING".into(), 4), "LDREOEIIECIHNTSG".to_owned());
    assert_eq!(Solution::convert("A".into(), 1), "A".to_owned());
    assert_eq!(Solution::convert("AB".into(), 1), "AB".to_owned());
    println!("ok")
}
