//Task №4 page 5
//Bertrand's postulate


use std::io::{BufRead, BufReader};
use std::fs::File;

fn is_prime(n: usize) -> bool {
    //Finding the divisor of a number in 
    //the range from two to the root of this number
    for i in 2..=((n as f64).sqrt() as usize) {
        if n % i == 0 {
            return false;
        }
    }
    true
}
fn read_my_file() -> Vec<usize>{
    let reader = BufReader::new(File::open("file.txt").expect("Cannot open file.txt"));
    let mut v: Vec<usize> = Vec::new();
    for line in reader.lines() {
        match line {
            Err(_e) => panic!("Reading error"),
            Ok(f) => v.push(f.parse::<usize>().unwrap()),
        }
    }
    if v[0] < 2 || v[0] > 1000000{
        panic!("Incorrect file data");
    }
    return v;
}

fn count_primes_in_range(n:usize) -> usize {
    let mut count = 0;
    for number in n..2*n {
        if is_prime(number) {
            count += 1;
        }
    }
    count
}

fn main() {
    let data: Vec<usize> = read_my_file();

    let prime_count = count_primes_in_range(data[0]);
    println!("The number of prime numbers in the range: {}", prime_count);
    if prime_count == data[1]{
        println!("The data matches the result in the source file");
    }
    else{ println!("The data does not match!!!"); }
    
}