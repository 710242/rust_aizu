use std::io;

fn main(){
    let mut index = 1;
    loop{
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer);
        let v = buffer.trim().parse::<i32>().unwrap();
        
        if v == 0{
            break;
        }
        
        println!("Case {}: {}",index,v);
        index = index+1;
        
    }
}
