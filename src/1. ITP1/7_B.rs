use std::io;

fn main(){
    loop{
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer);
        let mut nums :Vec<_> = buffer.split_whitespace().collect();
        
        let mut range = nums[0].parse::<i32>().unwrap();
        let mut sum = nums[1].parse::<i32>().unwrap();

        if range == 0 && sum == 0 {
            break;
        }
        
        let mut ans = 0;
        
        for i in (1..=range-2){
            for j in (i+1..=range-1){
                for k in (j+1..=range){
                    if i + j + k == sum{
                        // println!("{} + {} + {}",i,j,k);
                        ans +=1;
                    }
                }
            }
        }
        
        println!("{}",ans);
    }
}