#[test]
fn test_div_by_three() {
  if div_by_three(1) {
    fail!("One is not divisible by three!");
  }
}

#[test]
fn test_div_by_five() {
  if div_by_five(3) {
   fail!("Three is not divisible by five!");
  }
}

#[test]
fn test_div_by_fifteen() {
   if div_by_fifteen(14) {
    fail!("Fourteen is not divisible by fifteen!");
   }
}

fn div_by_three(num: int) -> bool {
  num % 3 == 0
}

fn div_by_five(num: int) -> bool {
  num % 5 == 0
}

fn div_by_fifteen(num: int) -> bool {
  div_by_three(num) && div_by_five(num)
}

fn main() {
  for num in range(1i, 101) {
    println!("{:s}", if div_by_fifteen(num) { "FizzBuzz".to_string() }
      else if div_by_three(num) { "Fizz".to_string() }
      else if div_by_five(num) { "Buzz".to_string() }
      else { num.to_string() }
    );
  }
}