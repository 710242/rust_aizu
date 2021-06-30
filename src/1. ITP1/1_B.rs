use std::io;

fn main(){
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let x: i32 = n.trim().parse().unwrap();
    println!("{}",x.pow(3));
}
