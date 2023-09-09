use std::io;
use std::fmt;

#[derive(Debug)]
enum LetterSumError {
    NonAsciiInput,
    UppercaseDetected,
}

impl fmt::Display for LetterSumError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LetterSumError::NonAsciiInput => write!(f, "Input should be ASCII! Please enter a valid string."),
            LetterSumError::UppercaseDetected => write!(f, "Input should be all lowercase! Please enter a valid string."),
        }
    }
}

fn main() {
    println!("Type in a lowercase ASCII message, and the letter sum of that string will be calculated.");

    loop {
        let mut user_input = String::new();
        if io::stdin().read_line(&mut user_input).is_err() {
            println!("Failed to read line. Please try again.");
            continue;
        }

        match lettersum(&user_input.trim()) {
            Ok(sum) => {
                println!("Sum: {}", sum);
                break;
            }
            Err(e) => println!("Error: {}", e),
        }
    }
}

fn lettersum(input: &str) -> Result<i32, LetterSumError> {
    for c in input.chars() {
        if !c.is_ascii() {
            return Err(LetterSumError::NonAsciiInput);
        }
        if c.is_uppercase() {
            return Err(LetterSumError::UppercaseDetected);
        }
    }

    let offset = 'a' as i32 - 1;
    let sum: i32 = input.bytes().map(|b| (b as i32) - offset).sum();
    Ok(sum)
}

#[test]
fn test_empty_string() {
    assert_eq!(lettersum("").unwrap(), 0);
}

#[test]
fn test_single_letter_a() {
    assert_eq!(lettersum("a").unwrap(), 1);
}

#[test]
fn test_single_letter_z() {
    assert_eq!(lettersum("z").unwrap(), 26);
}

#[test]
fn test_multiple_letters_cab() {
    assert_eq!(lettersum("cab").unwrap(), 6);
}

#[test]
fn test_word_excellent() {
    assert_eq!(lettersum("excellent").unwrap(), 100);
}

#[test]
fn test_long_word_microspectrophotometries() {
    assert_eq!(lettersum("microspectrophotometries").unwrap(), 317);
}

// You could also consider adding negative test cases.
#[test]
fn test_error_non_ascii_input() {
    assert!(lettersum("café").is_err());
}
#[test]
fn test_error_uppercase_detected() {
    assert!(lettersum("Hello").is_err());
}