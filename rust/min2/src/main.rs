use st::io;
fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();


    println!("enter thr first number");
  io::stdin().read_line(&mut input1).expect("Failed to read line");
  let first number: i32 = input1.trim().parse().expect("Please enter a valid number");


  println!("enter the second number");
  io::stdin().read_line(&mut input2).expect("Failed to read line");
  let second number: i32 = input2.trim().parse().expect("Please enter a valid number");

  
  if first number > second number {
    println!(the minimum number between first number: {},first number and second number: {},second number is second number: {},second number);

  } else {
    println!(the miinimum number between first number: {},first number and second number: {},second number is first number: {},first number);

  }



}
