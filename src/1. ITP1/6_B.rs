use std::io;

fn main(){

    let mut s = vec![false;13];
    let mut h = vec![false;13];
    let mut c = vec![false;13];
    let mut d = vec![false;13];
    
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer);
    let mut x = buffer.trim().parse::<usize>().unwrap();
    
    buffer.clear();
    
    for i in (0..x){
        io::stdin().read_line(&mut buffer);
        let mut card :Vec<_> = buffer.split_whitespace().collect();
        let mut num = card[1].trim().parse::<usize>().unwrap();
        
        match card[0]{
            "S" =>{
                s[num-1] = true;
            },
            "C" =>{
                c[num-1] = true;
            },
            "H" =>{
                h[num-1] = true;
            },
            "D" =>{
                d[num-1] = true;
            },
            _ => {
                break;
            }
        }
        buffer.clear();
    }
    
    for (i,w) in s.iter().enumerate(){
        if !*w{
            println!("{} {}","S",i+1);
        }
    }
    for (i,w) in h.iter().enumerate(){
        if !*w{
            println!("{} {}","H",i+1);
        }
    }
    for (i,w) in c.iter().enumerate(){
        if !*w{
            println!("{} {}","C",i+1);
        }
    }
    for (i,w) in d.iter().enumerate(){
        if !*w{
            println!("{} {}","D",i+1);
        }
    }
}