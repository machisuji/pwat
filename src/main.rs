extern crate rpassword;

use std::io::Write;
use std::io::stdout;
use rpassword::read_password;
use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() == 1 {
      println!("Usage: pwat m [n...]");
      println!("Prints bits of a password at the given 1-based indices.");
      println!("");
      println!("example: pwat 2 5 8");
      println!("         (typing in 'hallowelt')");
      println!(" result: aol");
  } else {
      print!("Enter password: ");
      stdout().flush();
      let password = read_password().unwrap();

      for s in args {
          for n in s.parse::<u32>() {
              for c in password.chars().nth(n as usize - 1) {
                  print!("{}", c);
              }
          }
      }
      println!("");
  }
}
