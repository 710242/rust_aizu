use std::io;

fn main(){
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer);
    let mut nums :Vec<_> = buffer.split_whitespace().collect();
    
    let mut a = nums[0].parse::<usize>().unwrap();
    let mut b = nums[1].parse::<usize>().unwrap();
    
    buffer.clear();

    let mut table = vec![vec![0;b+1];a+1];

    for i in (0..a){
        io::stdin().read_line(&mut buffer);
        let mut vect :Vec<_> = buffer.trim().split_whitespace().collect();
        for (index,j) in vect.iter().enumerate(){
            let n = j.parse::<i32>().unwrap();
            // println!("{} {}",i,index);
            table[i][index] = n;
            table[i][b] += n;
            table[a][index] += n;
            table[a][b] += n;
        }
        buffer.clear();
    }

    for i in (0..=a){
        print!("{}",table[i][0]);
        for j in (1..=b){
            print!(" {}",table[i][j]);
        }
        print!("\n");
    }

}