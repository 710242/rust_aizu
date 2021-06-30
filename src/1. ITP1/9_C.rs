use std::io;
use std::cmp::Ordering;

fn main(){

    let mut buffer = String::new();
    let mut buffer2 = String::new();
    io::stdin().read_line(&mut buffer);

    let mut round = buffer.trim().parse::<i32>().unwrap();

    let mut Taro = 0;
    let mut Hanako = 0;

    for i in (0..round){
        buffer2.clear();
        io::stdin().read_line(&mut buffer2);

        let mut cards :Vec<_>= buffer2.trim().split_whitespace().collect();
        // println!("{},{},{:?}",cards[0],cards[1],cards[0].cmp(&cards[1]));
        match cards[0].cmp(&cards[1]){
            Ordering::Less => {
                Hanako += 3;
            },
            Ordering::Greater => {
                Taro += 3;
            },
            Ordering::Equal => {
                Taro += 1;
                Hanako += 1;
            },
            _ => {}
        }

    }
    
    println!("{} {}",Taro,Hanako);

}