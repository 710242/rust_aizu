use std::io;
use std::cmp::Ordering;

fn main(){
    loop{
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer);
        let mut nums :Vec<_> = buffer.split_whitespace().collect();
        
        let a = nums[0].parse::<i32>().unwrap();
        let b = nums[1].parse::<i32>().unwrap();
        
        if a == 0 && b == 0{
            break;
        }else{
            match a.cmp(&b){
                Ordering::Greater => {
                    println!("{} {}",b,a);                    
                },
                Ordering::Less => {
                    println!("{} {}",a,b);                    
                },
                Ordering::Equal => {
                    println!("{} {}",b,a);                    
                },
            }
        }
        
    }
    
}
