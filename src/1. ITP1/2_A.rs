use std::io;
use std::cmp::Ordering;

fn main(){
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut nums :Vec<_> = buffer.split_whitespace().collect();
    let mut a = nums[0].parse::<i32>().unwrap();
    let mut b = nums[1].parse::<i32>().unwrap();
    
    // if a > b{
    //     println!("a > b");
    // }else if a < b {
    //     println!("a < b");
    // }else{
    //     println!("a == b");
    // }
    
    match a.cmp(&b){
        Ordering::Greater => {
            println!("a > b");
        },
        Ordering::Less => {
            println!("a < b");
        },
        Ordering::Equal => {
            println!("a == b");
        }
    }
    
}