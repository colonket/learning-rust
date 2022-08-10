pub mod test;

use rand::Rng;

fn roll_d(value: i32) -> i32{
    let result = rand::thread_rng().gen_range(1,value+1);
    return result;
}

fn main(){
    let mut trials: [i32; 100] = [0; 100];
    for i in 0..trials.len() {
        trials[i] = roll_d(6);
    }
    println!("{:?}",trials);
}