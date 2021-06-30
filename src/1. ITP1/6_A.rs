use std::io;

fn main(){
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer);
    let x = buffer.trim().parse::<usize>().unwrap();
    
    buffer.clear();
    
    io::stdin().read_line(&mut buffer);
    let mut nums :Vec<_> = buffer.split_whitespace().collect();
    
    for i in (0..x).rev(){
        if i == 0{
            let mut n = nums[i];
            print!("{}\n",&n);
        }else{
            let mut n = nums[i];
            print!("{} ",&n);
        }
    }
    
}