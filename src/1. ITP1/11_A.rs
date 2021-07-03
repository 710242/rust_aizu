use std::io;

fn main(){

    let mut buffer = String::new();
    let mut buffer2 = String::new();
    io::stdin().read_line(&mut buffer);

    let mut dice : Vec<_> = buffer.trim().split_whitespace().collect();

    io::stdin().read_line(&mut buffer2);

    let mut movements = buffer2.trim();

    //we only need to remember 3 face of dice by x+y=7 rule
    //respectively represent top ->  south -> east face
    let mut current_dice = vec![1,2,3];

    for i in movements.chars(){
        match i.to_string().as_ref(){
            "N" => {
                let temp = current_dice[0];
                current_dice[0] = current_dice[1];
                current_dice[1] = 7 - temp;
            },
            "S" => {
                let temp = current_dice[1];
                current_dice[1] = current_dice[0];
                current_dice[0] = 7 - temp;
            },
            "W" => {
                let temp = current_dice[0];
                current_dice[0] = current_dice[2];
                current_dice[2] = 7 - temp;
            },
            "E" => {
                let temp = current_dice[2];
                current_dice[2] = current_dice[0];
                current_dice[0] = 7 - temp;
            },
            _ => {}
        }
    }

    println!("{}",dice[current_dice[0]-1]);

}
