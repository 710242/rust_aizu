use std::io;

fn main(){

    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer);
    let mut times = buffer.trim().parse::<usize>().unwrap();

    buffer.clear();
    io::stdin().read_line(&mut buffer);

    let mut nums : Vec<_> = buffer.trim().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();

    for i in (1..nums.len()){

        for i in (0..nums.len()-1){
            print!("{} ", nums[i]);
        }
        println!("{}",nums[nums.len()-1]);

        let mut key = nums[i];
        // if i equal from 0, j will get error(usize cannot eq -n so turn to i32)
        let mut j = (i - 1) as i32;
        while j >= 0 && nums[j as usize] > key {
            nums[(j+1) as usize] = nums[j as usize];
            // if j is 0,next will end while loop
            j -= 1;
        }
        nums[(j+1) as usize] = key;
    }

    // print!("{}{}", x, if i == n - 1 { '\n' }
    for i in (0..nums.len()-1){
        print!("{} ", nums[i]);
    }
    println!("{}",nums[nums.len()-1]);

}
