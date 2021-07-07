use std::io;
use std::cmp::Ordering;

fn main(){

    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer);

    let mut nums : Vec<_> = buffer.trim().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();

    match nums[0].cmp(&nums[1]) {
        Ordering::Greater => {
            let smaeller = nums[1];
        },
        Ordering::Less => {

        },
        Ordering::Equal => {
            println!("{}",nums[0]);
        }
    }
}
