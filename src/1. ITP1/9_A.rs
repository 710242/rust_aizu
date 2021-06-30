use std::io;

fn main(){

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer);
    
    let mut target = buffer.trim();
    
    let mut cnt = 0;

    loop{

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer);
        let mut sentence :Vec<_>= buffer.trim().split_whitespace().collect();
        

        if buffer.trim() == String::from("END_OF_TEXT"){
            break;
        }

        for i in sentence{
            if i.to_lowercase() == target{
                cnt += 1;
            }
        }

        buffer.clear();

    }

    println!("{}",cnt);
}