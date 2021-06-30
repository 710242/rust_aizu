use std::io;

fn main(){

    loop{
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer);

        let mut nums = buffer.trim().parse::<f64>().unwrap();

        if nums == 0.{
            break;
        }

        buffer.clear();
        io::stdin().read_line(&mut buffer);

        let mut scores : Vec<_> = buffer.trim().split_whitespace().collect();

        let mut m = 0.;
        for i in &scores{
            m += i.parse::<f64>().unwrap();
        }
        m /= nums;

        let mut ans = 0.;
        for i in &scores{
            ans += (i.parse::<f64>().unwrap() - m ).powi(2);
        }
        ans /= nums;

        println!("{}",ans.sqrt());

    }

}
