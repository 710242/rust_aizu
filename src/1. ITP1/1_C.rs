use std::io;

fn main(){
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut iterator = buffer.split_whitespace();
    
    let a : i32 = iterator.next().unwrap().parse().unwrap();
    let b : i32 = iterator.next().unwrap().parse().unwrap();
    
    println!("{} {}",a*b,(a+b)*2);
}
