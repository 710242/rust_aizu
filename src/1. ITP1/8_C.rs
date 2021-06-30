use std::io;

fn main(){

    let mut strs = vec![0;26];
    let mut my_ascii_lower = String::from("abcdefghijklmnopqrstuvwxyz");
    
    loop{
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer);

        let ascii :Vec<_> = buffer.trim().split_whitespace().collect();

        if buffer.trim() == String::from(""){
            break;
        }

        for i in ascii.iter(){
            // split("") will return head and tail space
            // let v :Vec<_> = i.trim().split("").collect();
            let mut j = i.trim().chars();

            for index in (0..i.len()){
                let ch = j.next().unwrap().to_lowercase().to_string();
                if my_ascii_lower.contains(&ch){
                    strs[my_ascii_lower.find(&ch).unwrap()] += 1;
                }
            }
        }
    }

    let mut my_ascii = my_ascii_lower.chars();

    for index in (0..26){
        println!("{} : {}", my_ascii.next().unwrap(), strs[index]);
    }

}