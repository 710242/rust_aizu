use std::io;
use std::cmp::Ordering;

fn main(){
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut x :Vec<_> = buffer.split_whitespace().collect();
    
    let x1 = x[0].parse::<i32>().unwrap();
    let x2 = x[1].parse::<i32>().unwrap();
    let x3 = x[2].parse::<i32>().unwrap();
    
    if x1 < x2 && x2 < x3{
        println!("Yes");
    }else{
        println!("No");
    }
}
