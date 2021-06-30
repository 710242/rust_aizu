use std::io;

fn main(){

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer);
    
    let mut expr = buffer.trim();
    let mut expr = expr.repeat(2);

    // println!("{:?}",expr);

    let mut experiment = expr.chars();

    buffer.clear();

    io::stdin().read_line(&mut buffer);

    let mut target = buffer.trim();

    if expr.contains(target){
        println!("Yes");
    }else{
        println!("No");
    }

}