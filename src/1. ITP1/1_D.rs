use std::io;

fn main(){
    
    let mut buffer = String::new();
    
    io::stdin().read_line(&mut buffer).unwrap();
    
    let mut x :i32 = buffer.trim().parse().unwrap();
    
    let mut hours :i32 = x/3600;
    let mut mins :i32 = (x%3600)/60;
    let mut seconds :i32 = (x%60);
    
    println!("{}:{}:{}",hours,mins,seconds);
    
}