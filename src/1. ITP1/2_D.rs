use std::io;

fn main(){
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut nums : Vec<_> = buffer.split_whitespace().collect();
    let mut nums2 = Vec::new();
    for i in (0..nums.len()){
        nums2.push(nums[i].parse::<i32>().unwrap());
    }
    
    if nums2[2] + nums2[4] > nums2[0] || nums2[2] - nums2[4] < 0 || nums2[3] + nums2[4] > nums2[1] || nums2[3] - nums2[4] < 0{
        println!("No");
    }else{
        println!("Yes");
    }
}
