use std::env::{args, Args};
fn main() {
    let mut args: Args = args();
    let first: String = args.nth(1).unwrap();
    let operator: char = args.nth(0).unwrap().chars().next().unwrap();
    let second: String = args.nth(0).unwrap();
  
    println!("{} {} {}", first, operator, second);
    let first_number = first.parse::<i32>().unwrap();
    let second_number = second.parse::<i32>().unwrap();
      let result = operate(operator, first_number, second_number);
    println!("{}",result);
    }
  fn operate(operator:char , first_number : i32 , second_number : i32)-> i32 {
  match operator {
    '+' => first_number + second_number ,
    '-' =>   first_number - second_number ,
    '*' =>  first_number * second_number ,
    '/' => first_number / second_number,
    _ => panic! ("wrong operator bro")
    }
  }