use std::io;

fn main(){
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer);

    //in rust char is a iterator
    let mut ans = buffer.chars();

    for index in (0..buffer.len()){
        let i = ans.next().unwrap();
        
        if i.is_uppercase(){
            print!("{}",i.to_lowercase());
        }else if i.is_lowercase(){
            print!("{}",i.to_uppercase());
        }else{
            print!("{}",i);
        }
        
    }
    // print!("\n");

}