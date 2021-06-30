use std::io;

fn main(){
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer);
    let mut x = buffer.trim().parse::<f64>().unwrap();
    let pi = std::f64::consts::PI;

    println!("{} {}",x*x*pi,2.*pi*x);
}
