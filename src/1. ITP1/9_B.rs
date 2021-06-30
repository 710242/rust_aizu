use std::io;

fn main(){
    
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer);

    let mut expr = buffer.trim();        

    if buffer.trim() == String::from("-"){
        break;
    }

    //calculate
    let mut buffer2 = String::new();
    io::stdin().read_line(&mut buffer2);

    let mut movements = buffer2.trim().parse::<i32>().unwrap();

    let mut count :usize= 0;

    for i in (0..movements){
        buffer2.clear();
        io::stdin().read_line(&mut buffer2);
        count += buffer2.trim().parse::<usize>().unwrap();
    }

    count = count%expr.len();
    //

    if count == 0{
        println!("{}",expr);
    }else{
        println!("{}",expr.repeat(2).get(count..(expr.len()+count)).unwrap());
    }


}