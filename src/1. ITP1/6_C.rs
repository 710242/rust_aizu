use std::io;

fn main(){

    let mut building1 = vec![vec![0;10];3];
    let mut building2 = vec![vec![0;10];3];
    let mut building3 = vec![vec![0;10];3];
    let mut building4 = vec![vec![0;10];3];

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer);
    let mut x = buffer.trim().parse::<usize>().unwrap();
    
    buffer.clear();
    
    for i in (0..x){
        io::stdin().read_line(&mut buffer);
        let mut people_to_room :Vec<_> = buffer.split_whitespace().collect();
        let mut building = people_to_room[0];
        let mut floor = people_to_room[1].trim().parse::<usize>().unwrap();
        let mut room = people_to_room[2].trim().parse::<usize>().unwrap();
        let mut num = people_to_room[3].trim().parse::<i32>().unwrap();
        
        match building{
            "1" => {
                building1[floor-1][room-1] += num;
            },
            "2" => {
                building2[floor-1][room-1] += num;
            },
            "3" => {
                building3[floor-1][room-1] += num;
            },
            "4" => {
                building4[floor-1][room-1] += num;
            },
            _ => {
                break;
            }
        }
        
        buffer.clear();
        
    }
    
    for (i,b) in vec![building1,building2,building3,building4].iter().enumerate(){
        for j in b{
            for k in j{
                print!(" {}",k);
            }
            print!("\n");
        }
        if i == 3{
            break;
        }else{
            println!("####################");
        }
    }
    
}