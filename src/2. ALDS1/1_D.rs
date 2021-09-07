use std::io;

fn main() {
    let mut buffer1 = String::new();
    let mut buffer2 = String::new();

    let mut minus = Vec::<i64>::new();
    let mut max: i64;
    // this min is to filter minus
    // otherwise it will TLE
    let mut min: i64 = std::i64::MAX;
    let mut profit = std::i64::MIN;

    io::stdin().read_line(&mut buffer1);

    let mut times = buffer1.trim().parse::<i64>().unwrap();

    for i in (0..times) {
        io::stdin().read_line(&mut buffer2);
        let mut max = buffer2.trim().parse::<i64>().unwrap();

        if !minus.is_empty() {
            for j in (0..minus.len()) {
                if max - minus[j] > profit {
                    profit = max - minus[j];
                }
            }
        }

        if max < min {
            minus.push(max);
            min = max;
        }

        buffer2.clear();
    }
    println!("{}", profit);
}
