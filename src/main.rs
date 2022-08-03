use rand::Rng;

fn roll_d(value: i32) -> i32{
    let result = rand::thread_rng().gen_range(1,value-1);
    return result;
}

fn main(){
    println!("You rolled a d{val} and got: {roll}", val=6, roll=roll_d(val));
}