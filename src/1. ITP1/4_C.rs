use std::io;

fn main() {
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer);
        let mut nums :Vec<_>= buffer.split_whitespace().collect();

        let a = nums[0].parse::<i32>().unwrap();
        let b = nums[2].parse::<i32>().unwrap();

        match nums[1]{
            "+" => {
                println!("{}",a+b);
            },
            "-" => {
                println!("{}",a-b);
            },
            "*" => {
                println!("{}",a*b);
            },
            "/" => {
                println!("{}",a/b);
            },
            _ => {
                break;
            },
        }
    }
}
