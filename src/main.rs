// Kata
// REF: https://www.codewars.com/kata/50654ddff44f800200000004

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 3 {

        let operand1: i32 = args[1].parse().unwrap();
        let operand2: i32 = args[2].parse().unwrap();

        let c = multiply(operand1, operand2);
        println!("{}", c);
    } else {
        eprintln!("Usage: multiply 12 12 «operand1» «operand2»");
    }
}

fn multiply(a:i32, b:i32) -> i32 {
  a * b
}

#[test]
fn returns_expected() {
  assert_eq!(multiply(3, 5), 15);
  assert_eq!(multiply(3, 0), 0);
  assert_eq!(multiply(0, 5), 0);
  assert_eq!(multiply(-3, -5), 15);
  assert_eq!(multiply(-3, 5), -15);
  assert_eq!(multiply(3, -5), -15);
}
