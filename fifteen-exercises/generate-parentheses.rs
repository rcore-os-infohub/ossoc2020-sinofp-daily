impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        fn dfs(res: &mut Vec<String>, string: String, l: i32, r: i32, n: i32) {
            if r > l || r > n || l > n {
                return;
            }
            if r == l && r == n {
                res.push(string);
                return;
            }
            dfs(res, string.clone() + "(", l + 1, r, n);
            dfs(res, string + ")", l, r + 1, n);
        }
        let mut res = Vec::new();
        dfs(&mut res, "".to_owned(), 0, 0, n);
        res
    }
}

struct Solution;

fn main() {
    assert_eq!(Solution::generate_parenthesis(3), vec!["((()))".to_owned(),
                                                       "(()())".to_owned(),
                                                       "(())()".to_owned(),
                                                       "()(())".to_owned(),
                                                       "()()()".to_owned()]);
    println!("ok")
}
