use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer);
    let mut x = buffer.trim().parse::<usize>().unwrap();
    
    // without clear, nums will be [x,...]
    buffer.clear();
    io::stdin().read_line(&mut buffer);
    let mut nums :Vec<_> = buffer.split_whitespace().collect();
    
    let initiate = nums[0].trim().parse::<i64>().unwrap();
    let mut smallest = initiate;
    let mut largest = initiate;
    let mut sum = initiate;
    
    for i in(1..x){
        let n = nums[i].trim().parse::<i64>().unwrap();
        if n > largest{
            largest = n;
        }
        if n < smallest{
            smallest = n;
        }
        sum += n;
    }

    println!("{} {} {}",smallest,largest,sum);
    
}
