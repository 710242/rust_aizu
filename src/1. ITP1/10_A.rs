use std::io;

fn main(){

    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer);

    let mut position :Vec<_> = buffer.trim().split_whitespace().collect();

    let x1 :f64 = position[0].parse().unwrap();
    let y1 :f64 = position[1].parse().unwrap();

    let x2 :f64 = position[2].parse().unwrap();
    let y2 :f64 = position[3].parse().unwrap();

    let dis_x = (x2-x1).powi(2);
    let dis_y = (y2-y1).powi(2);

    println!("{}",(dis_x+dis_y).sqrt());


}
