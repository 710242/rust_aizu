use std::io;

#[derive(Clone)]
pub struct Dice{
    // top,front,right,left,back,bottom
    // 1,2,3,4,5,6
    order:Vec<usize>
}

impl Dice {
    fn new(top:usize,front:usize,right:usize,left:usize,back:usize,bottom:usize) -> Self{
        Dice{order:vec![top,front,right,left,back,bottom]}
    }

    fn swap(&mut self,x:usize,y:usize){
        let mut temp = self.order[x];
        self.order[x] = self.order[y];
        self.order[y] = temp;
        // (self.order[x] , self.order[y]) = (self.order[y] , self.order[x])
    }

    fn rotate_right(&mut self){
        self.swap(0, 3);
        self.swap(5, 3);
        self.swap(5, 2);
    }

    fn rotate_left(&mut self){
        self.swap(0, 2);
        self.swap(5, 2);
        self.swap(5, 3);
    }

    fn rotate_front(&mut self){
        self.swap(0, 1);
        self.swap(1, 5);
        self.swap(5, 4);
    }

    fn rotate_back(&mut self){
        self.swap(0, 4);
        self.swap(5, 4);
        self.swap(5, 1);
    }

    fn rotate_bottom(&mut self){
        self.swap(0, 5);
        self.swap(2, 3);
    }

    fn rotate_east(&mut self){
        self.swap(1, 3);
        self.swap(3, 4);
        self.swap(4, 2);
    }

    fn rotate_west(&mut self){
        self.swap(1, 2);
        self.swap(2, 4);
        self.swap(4, 3);
    }

    fn check_same(&mut self,other:Dice) -> bool{
        //this way we can only determine three face

        match other.order[0]{
            a if a == self.order[1] => {
                self.rotate_front();
            },
            a if a == self.order[2] => {
                self.rotate_left();
            },
            a if a == self.order[3] => {
                self.rotate_right();
            },
            a if a == self.order[4] => {
                self.rotate_back();
            },
            a if a == self.order[5] => {
                self.rotate_bottom();
            },
            _ => {},
        }

        let mut cnt = 0;

        while !self.order[1..5].eq(&other.order[1..5]) {
            if cnt > 5{
                return false;
            }
            self.rotate_east();
            cnt += 1;
        }

        return true;

    }

}

fn main(){

    let mut buffer = String::new();
    let mut buffer2 = String::new();

    io::stdin().read_line(&mut buffer);
    let mut times = buffer.trim().parse::<usize>().unwrap();

    let mut dices :Vec<Dice> = vec![];

    for i in (0..times){
        io::stdin().read_line(&mut buffer2);
        let mut dice : Vec<_> = buffer2.trim().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();
        let mut dice = Dice::new(dice[0], dice[1], dice[2], dice[3], dice[4], dice[5]);

        dices.push(dice);
        buffer2.clear();
    }

    for i in (0..dices.len()-1){
        if dices[i].check_same(dices[i+1].clone()) == false {
            println!("No");
            break;
        }
    }

    println!("Yes");

}
