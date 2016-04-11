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
      println!(" result: lwt");
  } else {
      print!("Enter password: ");
      stdout().flush();
      let password = read_password().unwrap();

      for s in args {
          for n in s.parse::<u32>() {
              print!("{}", password.chars().nth(n as usize).unwrap());
          }
      }
      println!("");
  }
}
