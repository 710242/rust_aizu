use std::io;

fn main(){
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut nums :Vec<_> = buffer.split_whitespace().collect();
    
    for i in (0..nums.len()){
        for j in (i..nums.len()){
            if nums[i] > nums[j]{
                let mut temp = nums[i];
                nums[i] = nums[j];
                nums[j] = temp;
            }
        }
    }
    
    println!("{} {} {}",&nums[0],&nums[1],&nums[2]);
    
}
