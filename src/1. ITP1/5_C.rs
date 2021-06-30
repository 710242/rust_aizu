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
                if i % 2 == 0{
                    if j % 2 == 0{
                        print!("#");
                    }else{
                        print!(".");
                    }
                }else{
                    if j % 2 == 0{
                        print!(".");
                    }else{
                        print!("#");
                    }
                }
            }
            print!("\n");
        }
        print!("\n");
    }
}
