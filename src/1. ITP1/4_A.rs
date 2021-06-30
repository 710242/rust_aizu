use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer);
    let mut nums :Vec<_> = buffer.split_whitespace().collect();
    
    let a1 = nums[0].parse::<i32>().unwrap();
    let b1 = nums[1].parse::<i32>().unwrap();
    
    let a2 = nums[0].parse::<f64>().unwrap();
    let b2 = nums[1].parse::<f64>().unwrap();
    
    println!("{} {} {}",a1/b1,a1%b1,a2/b2);
    
}
