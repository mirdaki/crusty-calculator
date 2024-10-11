use calc::{divide, hello, multiply, subtract, sum};

fn main() {
    println!("{}", hello("world"));

    println!("1 + 2: {}", sum(1, 2));
    println!("1 - 2: {}", subtract(1, 2));
    println!("1 * 2: {}", multiply(1, 2));
    println!("1 / 2: {}", divide(1, 2));
}
