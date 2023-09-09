use std::io;

// Define a custom error type for more descriptive errors
#[derive(Debug)]
pub enum LetterSumError {
    NonAsciiInput,
    UppercaseDetected,
}

fn main() {
    println!("Type in a ascii message and the lettersum of that string will be calculated.");

    loop {
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        match lettersum(&user_input.trim()) {
            Ok(sum) => {
                println!("Sum: {}", sum);
                break; // Exit the loop once we have a valid sum
            }
            Err(LetterSumError::NonAsciiInput) => println!("Error: Input should be ASCII! Please enter a valid string."),
            Err(LetterSumError::UppercaseDetected) => println!("Error: Input should be all lowercase! Please enter a valid string."),
        }
    }
    
}


pub fn lettersum(input: &str) -> Result<i32, LetterSumError> {
    if !input.is_ascii() {
        return Err(LetterSumError::NonAsciiInput);
    }
    if input.chars().any(char::is_uppercase) {
        return Err(LetterSumError::UppercaseDetected);
    }

    // Your logic to compute the letter sum
    let offset = 'a' as i32 - 1;
    let sum: i32 = input.bytes().map(|b| (b as i32) - offset).sum();

    return Ok(sum);

}

#[test]
fn it_works() {
    assert_eq!(lettersum("").unwrap(), 0);

    assert_eq!(lettersum("a").unwrap(), 1);

    assert_eq!(lettersum("z").unwrap(), 26);

    assert_eq!(lettersum("cab").unwrap(), 6);

    assert_eq!(lettersum("excellent").unwrap(), 100);

    assert_eq!(lettersum("microspectrophotometries").unwrap(), 317);
}
