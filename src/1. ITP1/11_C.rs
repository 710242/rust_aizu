use std::io;

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
    let mut dice1 : Vec<_> = buffer.trim().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();
    let mut dice1 = Dice::new(dice1[0], dice1[1], dice1[2], dice1[3], dice1[4], dice1[5]);

    io::stdin().read_line(&mut buffer2);
    let mut dice2 : Vec<_> = buffer2.trim().split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect();
    let mut dice2 = Dice::new(dice2[0], dice2[1], dice2[2], dice2[3], dice2[4], dice2[5]);

    match dice1.check_same(dice2) {
        true => {
            println!("Yes");
        },
        false => {
            println!("No");
        }
    }

}
