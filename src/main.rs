use std::io;

fn main(){

    let mut buffer = String::new();
    let mut buffer2 = String::new();

    io::stdin().read_line(&mut buffer);

    let mut expr = buffer.trim();

    io::stdin().read_line(&mut buffer2);
    let mut round = buffer2.trim().parse::<i32>().unwrap();

    for i in (0..round){
        buffer2.clear();
        io::stdin().read_line(&mut buffer2);

        let mut movement :Vec<_>= buffer2.trim().split_whitespace().collect();

        let mut start = movement[1].parse::<usize>().unwrap();
        let mut end = movement[2].parse::<usize>().unwrap()+1;

        match movement[0]{
            "print" => {
                println!("{}",expr.get(start..end).unwrap());
            },
            "replace" => {
                let mut new = movement[3];
                expr.replace(expr.get(start..end).unwrap(), new);
            },
            "reverse" => {
                let mut reverse = expr.get(start..end).unwrap().chars().rev().collect::<String>();
                expr.replace(expr.get(start..end).unwrap(), reverse.as_ref());
            },
            _ => {}
        }

    }

}