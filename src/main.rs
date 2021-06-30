use std::io;

fn main(){

    let mut buffer = String::new();
    let mut buffer2 = String::new();
    io::stdin().read_line(&mut buffer);

    let mut dice : Vec<_> = buffer.trim().split_whitespace().collect();

    io::stdin().read_line(&mut buffer2);

    let mut movements = buffer2.trim();



}
