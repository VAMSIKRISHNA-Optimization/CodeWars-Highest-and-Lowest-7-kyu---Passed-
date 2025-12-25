# CodeWars-Highest-and-Lowest-7-kyu---Passed-

In this little assignment you are given a string of space separated numbers, and have to return the highest and lowest number.

Examples
high_and_low("1 2 3 4 5") // return "5 1"
high_and_low("1 2 -3 4 5") // return "5 -3"
high_and_low("1 9 3 4 -5") // return "9 -5"
Notes
All numbers are valid Int32, no need to validate them.
There will always be at least one number in the input string.
Output string must be two numbers separated by a single space, and highest number is first.


// The rusts stdlib doesn't include functions for random number generation. 
// This typically provided by the “rand” external library, we unfortunately 
// don’t have access to external libraries. We get around this by linking to
// rand() in the C standard library.
extern {
  fn rand() -> isize;
}

// Safe wrapper for rand() 
fn crand() -> isize {
  unsafe { rand() as isize }
}

fn crand_range(range: std::ops::Range<isize>) -> isize {
  crand() % (range.end - range.start) + range.start
}

#[test]
fn random_tests() {
  for _ in 0..20 {
    let n = (0..crand_range(1..40)).map(|_|{
      crand_range(0..1024) - 512
    }).collect::<Vec<_>>();
    
    let min_max = format!("{} {}",
      n.iter().max().unwrap(),
      n.iter().min().unwrap()
    );
    
    let mut s = String::new();
    for i in n.iter().enumerate() {
      if i.0 != 0 {
        s.push(' ');
      }
      s.push_str(&i.1.to_string()[..])
    }
    
    assert_eq!(min_max, high_and_low(&s[..]));
  }
}

#[test]
fn some_test() {
  assert_eq!("542 -214", high_and_low("4 5 29 54 4 0 -214 542 -64 1 -3 6 -6"));
}

#[test]
fn sort_test() {
  assert_eq!("10 -20", high_and_low("10 2 -1 -20"));
}

#[test]
fn plus_minus_test() {
  assert_eq!("1 -1", high_and_low("1 -1"));
}

#[test]
fn plus_plus_test() {
  assert_eq!("1 1", high_and_low("1 1"));
}

#[test]
fn minus_minus_test() {
  assert_eq!("-1 -1", high_and_low("-1 -1"));
}

#[test]
fn plus_minus_zero_test() {
  assert_eq!("1 -1", high_and_low("1 -1 0"));
}

#[test]
fn plus_plus_zero_test() {
  assert_eq!("1 0", high_and_low("1 1 0"));
}

#[test]
fn minus_minus_zero_test() {
  assert_eq!("1 0", high_and_low("1 1 0"));
}

#[test]
fn single_test() {
  assert_eq!("42 42", high_and_low("42"));
}
