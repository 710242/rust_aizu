use std::io;

fn main(){
    loop{
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).ok();

        //this trim is to make the \n disappear
        if buffer.trim() == String::from("0"){
            break;
        }

        //this trim() shall be added becaz it mau cause panic when do unwrap() -> Err
        let mut num = buffer.trim().chars();
        let mut sum = 0;

        loop{
            match num.next(){
                Some(ref r) =>{
                    sum += r.to_string().parse::<i64>().unwrap();
                },
                None => {break;}
            }
        }
        
        println!("{}",sum);

    }
}