use std::io;
use ramp::Int;

//can calculates the fibonacci number, up to memory limits. uses ramp on nightly to get really big values.
fn main() {
    println!("Input Desired Fibonacci Number");

    let mut fibnum = String::new();

    io::stdin()
        .read_line(&mut fibnum)
        .expect("Failed to read line");

    let fibnum: usize = match fibnum.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Bad Input!");
            0
        }
    };
    let mut array: [Int; 2] = [Int::zero(), Int::one()];
    if fibnum == 0 || fibnum == 1 {
        println!("Fibnumber = {}", array[fibnum]);
    } else {
        let mut counter = 2;
        let mut num: Int = Int::zero();
        while counter <= fibnum {
            num = array[0].clone() + array[1].clone();
            array[0] = array[1].clone();
            array[1] = num.clone();
            counter += 1;
        }

        println!("Fibnumber = {}", num);
    }
}

