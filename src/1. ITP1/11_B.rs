use std::io;

fn detection(x:usize,y:usize,r:bool) -> usize{
    let mut front_right = match r {
        false => {
            vec![
                //top is 1,2,3
                vec![2,3],
                vec![3,1],
                vec![1,2]
            ]
        },
        true => {
            vec![
                //top is 6,5,4
                vec![3,2],
                vec![1,3],
                vec![2,1]
            ]
        },
    };

    while front_right[x-1][0] != y {
        let mut temp = front_right[x-1][0];
        front_right[x-1].remove(0);
        front_right[x-1].push(7-temp);
    }

    return front_right[x-1][1];

}

fn main(){

    let mut buffer = String::new();
    let mut buffer2 = String::new();
    io::stdin().read_line(&mut buffer);

    let mut dice : Vec<_> = buffer.trim().split_whitespace().collect();

    io::stdin().read_line(&mut buffer2);

    let mut times = buffer2.trim().parse::<usize>().unwrap();

    for i in (0..times){
        buffer2.clear();
        io::stdin().read_line(&mut buffer2);
        let mut question : Vec<_> = buffer2.trim().split_whitespace().collect();

        let mut top = dice.iter().position(|x| x == &question[0]).unwrap() + 1;
        let mut front = dice.iter().position(|x| x == &question[1]).unwrap() + 1;
        let mut reverse = false;

        if  top > 3{
            top = 7 - top;
            reverse = true;
        }

        println!("{}",dice[detection(top, front, reverse) - 1]);
    }

}
