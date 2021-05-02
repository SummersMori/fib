use std::io;
use uint::construct_uint;
//can calculate up to the 2951st fibonacci number. any larger an it takes too long to compile
//any edits at all causes it to recompile. even comments.
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
    let mut one: [u64; 32] = [0; 32];
    one[0] = 1;
    let mut array: [U1024; 2] = [U1024([0; 32]), U1024(one)]; //lowest bit first
    if fibnum == 0 || fibnum == 1 {
        println!("Fibnumber = {}", array[fibnum]);
    } else {
        let mut counter = 2;
        let mut num: U1024 = U1024([0; 32]);
        while counter <= fibnum {
            num = array[0] + array[1];
            array[0] = array[1];
            array[1] = num;
            counter += 1;
        }

        println!("Fibnumber = {}", num);
    }
}

// U1024 with 1024 bits consisting of 32 x 64-bit words
construct_uint! {
    pub struct U1024(32);
}
