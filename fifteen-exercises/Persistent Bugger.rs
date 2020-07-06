fn persistence(num: u64) -> u64 {
    let mut cnt = 0;
    let mut numstr = num.to_string();
    while numstr.len() > 1 {
        numstr = numstr
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .fold(1,|a, b| a * b)
            .to_string();
        cnt += 1
    }
    cnt
}

fn main() {
    assert_eq!(persistence(39), 3);
    assert_eq!(persistence(999), 4);
    assert_eq!(persistence(4), 0);
    println!("ok")
}