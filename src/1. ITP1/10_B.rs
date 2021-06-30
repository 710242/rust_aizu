use std::io;

fn main(){

    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer);

    let mut info :Vec<_> = buffer.trim().split_whitespace().collect();

    let side_a :f64 = info[0].parse().unwrap();
    let side_b :f64 = info[1].parse().unwrap();
    let angel :f64 = info[2].parse::<f64>().unwrap()/180.*std::f64::consts::PI;

    let area : f64 = side_a*side_b*0.5*angel.sin();
    let side_c :f64 = (side_a.powi(2) + side_b.powi(2) - 2.*side_a*side_b*angel.cos()).sqrt();

    println!("{}",area);
    println!("{}",side_a+side_b+side_c);
    println!("{}",side_b*angel.sin());

}
