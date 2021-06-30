use std::io;

fn main(){
    loop{
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer);
        let mut grade :Vec<_> = buffer.split_whitespace().collect();
        
        if grade[0] == "-1" && grade[1] == "-1" && grade[2] == "-1" {
            break;
        }
        
        let mut mid = grade[0].parse::<i32>().unwrap();
        let mut fin = grade[1].parse::<i32>().unwrap();
        let mut makeup = grade[2].parse::<i32>().unwrap();
        
        if mid == -1 || fin == -1{
            println!("F");
        }else{
            if mid + fin >= 80{
                println!("A");
            }else if mid + fin >= 65 && mid + fin < 80{
                println!("B");
            }else if mid + fin >= 50 && mid + fin < 65{
                println!("C");
            }else if mid + fin >= 30 && mid + fin < 50{
                if makeup >= 50{
                    println!("C");
                }else{
                    println!("D");
                }
            }else if mid + fin < 30{
                println!("F");
            }
        }
        
        
    }
}