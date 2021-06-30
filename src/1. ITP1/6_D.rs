use std::io;

fn main(){
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer);
    let mut matrix :Vec<_> = buffer.split_whitespace().collect();
    let mut a_l = matrix[0].trim().parse::<usize>().unwrap();
    let mut b_l = matrix[1].trim().parse::<usize>().unwrap();
    
    buffer.clear();

    let mut a_m :Vec<Vec<_>> = vec![vec![0;b_l];a_l];
    let mut b_m = Vec::with_capacity(b_l);

    for i in (0..a_l){
        io::stdin().read_line(&mut buffer);
        let mut vect :Vec<_> = buffer.split_whitespace().collect();
        for (index,j) in vect.iter().enumerate(){
            a_m[i][index] = j.parse::<i32>().unwrap();
        }
        buffer.clear();
    }
    
    for i in (0..b_l){
        io::stdin().read_line(&mut buffer);
        let mut vect  = buffer.trim().parse().unwrap();
        b_m.push(vect);
        buffer.clear();
    }
    
    for i in (0..a_l){
        let mut j = a_m[i].iter().zip(b_m.iter());
        let mut multiply = 0;
        for (x,y) in j{
            multiply += x*y;
        }
        println!("{}",multiply);
    }
    
}