
pub fn fizz_buzz(n: Option<u32>) {
    
    let mut default = 100u32;
    if let Some(i) = n {
        default = i;
    }


    for i in 1..=default {
        if i % 15 == 0 {
            println!("FizzBuzz")
        }
        else if i % 3 == 0 {
            println!("Fizz")
        }
        else if i % 5 == 0 {
            println!("Buzz")
        }
        else {
            println!("{i}")
        }
    }
}
