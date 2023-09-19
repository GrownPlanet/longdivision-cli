use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (a, b) = longdiv(args[1].parse::<i32>().unwrap(), args[2].parse::<i32>().unwrap());

    println!("{} R {}", a, b);
}

fn longdiv(a: i32, b: i32) -> (i32, i32) {
    return ((a/b) as i32, a % b);
}


