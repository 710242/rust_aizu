use std::io;

fn main() {
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer);
        let mut nums :Vec<_> = buffer.split_whitespace().collect();

        let a = nums[0].trim().parse::<i32>().unwrap();
        let b = nums[1].trim().parse::<i32>().unwrap();
        
        if a == 0 && b == 0{
            break;
        }
        
        for i in (0..a){
            for j in (0..b){
                print!("#");    
            }
            print!("\n");
        }
        print!("\n");
    }
}
