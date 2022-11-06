use std::io;

fn main() {
    println!("Enter one number ");
    println!("or 'E' if you want to exit");
    
    loop {
    
        let mut any_number = String::new();

        io::stdin().read_line(&mut any_number)
            .expect("Failed to read line");

        if any_number.to_uppercase().trim() == "E" {
            break;
        }

        let any_number: i32 = match any_number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let value: i32;
        let other_value: i32 = any_number.clone();

        value = fib_algorithm_one(other_value);

        println!("fib_algorithm_one({}) => {}", other_value, value);

        let value_float: f32 = fib_algorithm_two(other_value);

        println!("fib_algorithm_two({}) => {}", other_value, value_float);
    }

}


fn fib_algorithm_one(n: i32) -> i32 {
    if n < 2 {
        return n;
    } else {
        return fib_algorithm_one(n - 1) + fib_algorithm_one(n -2);
    }
}

fn fib_algorithm_two(n: i32) -> f32 {
    if n < 2 {
        return n as f32;
    } else {
        let p: f32 = (1.0 + 5.0_f32.sqrt())/2.0;
        let j: f32 = (p.powf(n as f32) - (1.0 -p).powf(n as f32)) / 5.0_f32.sqrt();
        return j;
    }
}