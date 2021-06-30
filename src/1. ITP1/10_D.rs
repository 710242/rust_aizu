use std::io;

fn main(){

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer);

    let mut n = buffer.trim().parse::<usize>().unwrap();

    buffer.clear();
    io::stdin().read_line(&mut buffer);

    let mut x : Vec<_> = buffer.trim().split_whitespace().collect();

    let mut buffer2 = String::new();
    io::stdin().read_line(&mut buffer2);

    let mut y : Vec<_> = buffer2.trim().split_whitespace().collect();

    let mut p1 = 0.;
    for i in (0..n){
        p1 += (x[i].parse::<f64>().unwrap() - y[i].parse::<f64>().unwrap()).abs();
    }

    let mut p2 = 0.;
    for i in (0..n){
        p2 += (x[i].parse::<f64>().unwrap() - y[i].parse::<f64>().unwrap()).powi(2);
    }

    let mut p3 = 0.;
    for i in (0..n){
        p3 += (x[i].parse::<f64>().unwrap() - y[i].parse::<f64>().unwrap()).abs().powi(3);
    }

    let mut px = 0.;
    for i in (0..n){
        let temp = (x[i].parse::<f64>().unwrap() - y[i].parse::<f64>().unwrap()).abs();
        if px <= temp{
            px = temp;
        }
    }

    println!("{}",p1);
    println!("{}",p2.sqrt());
    println!("{}",p3.cbrt());
    println!("{}",px);

}
