use std::io;

fn main() {
    let mut buffer1 = String::new();
    let mut buffer2 = String::new();

    io::stdin().read_line(&mut buffer1);

    let mut times = buffer1.trim().parse::<i32>().unwrap();

    let mut cnt = 0;

    for i in (0..times) {
        io::stdin().read_line(&mut buffer2);
        let mut num = buffer2.trim().parse::<i64>().unwrap();
        if check(num) {
            cnt += 1;
        }
        buffer2.clear();
    }
    println!("{}", cnt);
}

fn check(x: i64) -> bool {
    let mut x_sq = (x as f64).sqrt() as i64;
    for i in (2..x_sq + 1) {
        if (x % i == 0) {
            return false;
        }
    }
    return true;
}
