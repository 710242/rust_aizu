use std::io;

fn main(){
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer);
    let mut nums :Vec<_> = buffer.split_whitespace().collect();
    
    let mut n = nums[0].parse::<usize>().unwrap();
    let mut m = nums[1].parse::<usize>().unwrap();
    let mut l = nums[2].parse::<usize>().unwrap();
    
    buffer.clear();

    let mut a = vec![vec![0;m];n];
    let mut b = vec![vec![0;m];l];

    for i in (0..n){
        io::stdin().read_line(&mut buffer);
        let mut vect :Vec<_> = buffer.trim().split_whitespace().collect();
        for (index,j) in vect.iter().enumerate(){
            let num = j.parse::<i64>().unwrap();
            a[i][index] = num;
        }
        buffer.clear();
    }

    for i in (0..m){
        io::stdin().read_line(&mut buffer);
        let mut vect :Vec<_> = buffer.trim().split_whitespace().collect();
        for (index,j) in vect.iter().enumerate(){
            let num = j.parse::<i64>().unwrap();
            b[index][i] = num;
        }
        buffer.clear();
    }

    for i in a.iter(){
        for (index,j) in b.iter().enumerate(){

            let mut k = i.iter().zip(j.iter());
            let mut mult = 0;
            for (x,y) in k{
                mult += x*y;
            }

            if index == 0{
                print!("{}",mult);
            }else{
                print!(" {}",mult);
            }
            
        }
        print!("\n");
    }
}