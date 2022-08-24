use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn input_and_output() {
    println!("What is your name?");
    let mut name = String::new();
    let greeting = "Nice to meet you";
    io::stdin().read_line(&mut name).expect("Didn't recieve input");
    println!("Hello {}! {}", name.trim_end(), greeting);
}

fn consts_and_shadowing() {
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let age = "47";
    let mut age: u32 = age.trim().parse()
        .expect("Age wasn't assigned a number");
    age = age + 1;
    println!("I'm {} and I want ${}", age, ONE_MIL);
}
 fn max_sizes(){
    println!("Max u32 : {}", u32::MAX);
    println!("Max u64 : {}", u64::MAX);
    println!("Max usize : {}", usize::MAX);
    println!("Max u128 : {}", u128::MAX);
    println!("Max f32 : {}", f32::MAX);
    println!("Max f64 : {}", f64::MAX);
 }

 fn bool_and_char() {
    let is_true: bool = true;
    let my_grade = 'A';
 }

 fn math_ops_and_precision() {
    let num_1: f32 = 1.111111111111;
    println!("f32 : {}",num_1 + 0.111111111111111);

    let num_2: f64 = 1.111111111111;
    println!("f64 : {}",num_2 + 0.1111111111111111);

    let num_3: u32 = 5;
    let num_4: u32 = 4;
    println!("5 + 4 = {}", num_3 + num_4);
    println!("5 - 4 = {}", num_3 - num_4);
    println!("5 * 4 = {}", num_3 * num_4);
    println!("5 / 4 = {}", num_3 / num_4);
    println!("5 % 4 = {}", num_3 % num_4);
 }

 fn gen_random_int() {
    // Print random number from 1 - 100, including 100
    let random_num = rand::thread_rng().gen_range(1..101);
    println!("Random: {}", random_num);
 }

 fn if_else() {
    println!("How old are you?");
    let mut age_in = String::new();
    io::stdin().read_line(&mut age_in).expect("Unable to read age");
    let age: u32 = age_in.trim().parse().expect("No num given to age");

    if (age >= 1) && (age <= 18){
        println!("Important Birthday");
    } else if (age == 21) || (age == 50){
        println!("Important Birthday");
    } else if age >= 65 {
        println!("Important Birthday");
    } else {
        println!("Not an important birthday");
    }
 }

 fn ternary_ops(){
    let my_age = 22;
    let can_vote = if my_age >= 18{
        true
    } else {
        false
    };
    println!("Can vote: {}", can_vote);

    let age2: i32 = 65;
    match age2 {
        1..=18 => println!("Important Birthday"),
        21 | 50 => println!("Important Birthday"),
        65..=i32::MAX => println!("Important Birthday"),
        _ => println!("Not an Important Birthday"),
    };
 }

 fn match_and_comparisons() {
    let my_age = 11;
    let voting_age = 18;
    match my_age.cmp(&voting_age){
        Ordering::Less => println!("Can't vote"),
        Ordering::Greater => println!("Can vote"),
        Ordering::Equal => println!("You have gained the right to vote"),
    };
 }

 fn loop_arr(){
    let arr_1 = [1,2,3,4,5,6,7,8,9];
    println!("1st: {}", arr_1[0]);
    println!("Length: {}", arr_1.len());

    let mut loop_idx = 0;
    loop {
        // Loops through arr_1
        // and only prints odd numbers
        if arr_1[loop_idx] % 2 == 0 {
            loop_idx += 1;
            continue;
        }
        println!("Val: {}", arr_1[loop_idx]);
        if arr_1[loop_idx] >= arr_1.len() - 1{
            break;
        }
        loop_idx += 1;
    }

 }

 fn for_loop(){
    let arr_2 = [1,2,3,4,5,6,7,8,9];
    let mut loop_idx = 0;
    
    // Print all items of arr_2
    while loop_idx < arr_2.len() {
        println!("Arr: {}", arr_2[loop_idx]);
        loop_idx += 1;
    }

    for val in arr_2.iter() {
        println!("Val: {}",val);
    }
 }

fn tuples() {
    let my_tuple: (u8, String, f64) = (47, "Derek".to_string(), 50_000.00);

    println!("Name: {}",my_tuple.1);
    let(v0,v1,v2) = my_tuple;
    println!("Age: {}", v0);
}

fn strings() {
    let mut st1 = String::new();
    st1.push('A');
    st1.push_str(" word");
    for word in st1.split_whitespace(){
        println!("{}",word);
    }
    let st2 = st1.replace("A", "Another");
    println!("{}", st2);

    let st3 = String::from("x r t b h k k a m c");
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup();
    for char in v1 {
        println!("{}",char);
    }
    let st4: &str = "some string";
    let mut st5: String = st4.to_string();
    println!("{}", st5);
    let byte_arr1 = st5.as_bytes();
    let st6 = &st5[0..6];
    println!("String length: {}", st6.len());
    st5.clear();
    let st6 = String::from("Just some");
    let st7 = String::from(" words");
    let st8 = st6 + &st7;
    for char in st8.bytes(){
        println!("{}", char);
    }
}

fn main() {
    
}