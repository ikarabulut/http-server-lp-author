  fn main() {
  let hello : String = String::from("Hello");
  take(hello); // From take(): Hello WasmEdge!
  // The following will fail since hello is already taken by take() and no longer available here
  // println!("From main(): {}", hello);

  let hello : String = String::from("Hello");
  take(hello.clone()); // From take(): Hello WasmEdge!
  println!("From main(): {}", hello); // From main(): Hello
                                      
// In the borrow() function, we would like the changes made to the string to propagate back out of the borrow() function.
// That is, when we print the hello variable after the borrow() call, it should print the updated value. 
// Change the Rust source code to achieve this effect. Submit the updated source code as a deliverable for this milestone.

  let mut hello : String = String::from("Hello");
  borrow(&mut hello); // From borrow(): Hello WasmEdge!
  println!("From main(): {}", hello); // From main(): Hello
}

fn take (mut s: String) {
  s.push_str(" WasmEdge!");
  println!("From take(): {}", s);
}

fn borrow (s: &mut String) {
  (*s).push_str(" WasmEdge!");
  println!("From borrow(): {}", s);
}
