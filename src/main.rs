use std::env;

fn main()
{
    let args: Vec<String> = env::args().collect();

    if args.len() != 3
    {
        println!("please input 2 numbers");
        return;
    }
    
    if args[1].parse::<f64>().is_err() ||
       args[2].parse::<f64>().is_err()
    {
        println!("inputs are not numbers");
        return;
    }

    let (a, b) = longdiv
    (
        args[1].parse::<f64>().unwrap(),
        args[2].parse::<f64>().unwrap()
    );

    println!("{} R {}", a, b);
}

fn longdiv(a: f64, b: f64) -> (i32, i32)
{
    return ((a/b) as i32, (a%b) as i32);
}


