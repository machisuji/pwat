extern crate rpassword;

use std::io::Write;
use std::io::stdout;
use rpassword::read_password;
use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() == 1 {
      print_usage();
  } else {
      print!("Enter password: ");
      flush();
      let password = read_password().unwrap();
      let chars = get_chars(&password, parse_indices(&args));
      let result: String = chars.into_iter().collect();

      println!("{}", result);
  }
}

/**
 * Gets a string's chars at the given 1-based indices.
 * Out-of-bounds indices will be ignored and not included
 * in the result.
 */
fn get_chars(string: &String, indices: Vec<usize>) -> Vec<char> {
    return indices.into_iter().flat_map(|i| string.chars().nth(i - 1)).collect::<Vec<char>>();
}

/**
 * Tries parsing the given strings to numbers. Strings where this failed
 * will simply be dropped.
 */
fn parse_indices(xs: &Vec<String>) -> Vec<usize> {
    return xs.into_iter().flat_map(|s| s.parse::<usize>()).collect();
}

fn print_usage() {
    println!("Usage: pwat m [n...]");
    println!("Prints bits of a password at the given 1-based indices.");
    println!("");
    println!("example: pwat 2 5 8");
    println!("         (typing in 'hallowelt')");
    println!(" result: aol");
}

fn flush() {
    match stdout().flush() {
        Ok(_) => (),
        Err(_) => println!("")
    }
}

#[test]
fn parsing_indices() {
    let xs: Vec<String> = ["2", "5", "8"].into_iter().map(|s| s.to_string()).collect();
    let result: Vec<usize> = parse_indices(&xs);
    assert_eq!(result, [2, 5, 8]);
}

#[test]
fn getting_chars() {
    let string: String = "hallowelt".to_string();
    let indices: Vec<usize> = vec![2, 5, 8];
    let result: Vec<char> = get_chars(&string, indices);

    assert_eq!(result, ['a', 'o', 'l']);
}
