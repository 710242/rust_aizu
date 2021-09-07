use std::cmp::Ordering;
use std::io;

fn main() {
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer);

    let mut nums: Vec<_> = buffer
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let ans = f(nums[0], nums[1]);
    println!("{}", ans);
}

fn f(a: usize, b: usize) -> usize {
    if (b == 0) {
        return a;
    }
    return f(b, a % b);
}
