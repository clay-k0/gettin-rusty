// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None;
    }
    Some(a / b)
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{}{}", first, second)
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn check_lower_clamp() {
        let result = clamp(10, 100, 1000);
        let expected = 100;
        assert_eq!(result, expected, "Result should be 100.");
    }

    #[test]
    fn check_upper_clamp() {
        let result = clamp(1000, 56, 193);
        let expected = 193;
        assert_eq!(result, expected, "Result should be 193.");
    }

    #[test]
    fn check_div() {
        let result = div(20, 2);
        let expected = Some(10);
        assert_eq!(result, expected, "Result should be Some(10).");
    }

    #[test]
    fn check_div2() {
        let result = div(16, 4);
        let expected = Some(4);
        assert_eq!(result, expected, "Result should be Some(4).");
    }

    #[test]
    fn check_concat() {
        let result = concat("+", "-");
        let expected = String::from("+-");
        assert_eq!(result, expected, "Result should be +-");
    }

    #[test]
    fn check_concat2() {
        let result = concat("<", ">");
        let expected = String::from("<>");
        assert_eq!(result, expected, "Result should be <>.")
    }
}
