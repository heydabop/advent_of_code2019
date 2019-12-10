fn main() {
    let min = 172851;
    let max = 675869;

    let mut total = 0;

    for code in min..=max {
        let mut double_digits = Vec::new();
        let mut increasing = true;

        let mut digits = Vec::new();
        get_digits(code, &mut digits);

        for i in 1..digits.len() {
            let digit = &digits[i];
            let last_digit = &digits[i - 1];
            if digit < last_digit {
                increasing = false;
                break;
            }
            if digit == last_digit {
                if i > 1 && last_digit == &digits[i - 2] {
                    // This is actually more than a double, remove the last "double digit" we added if we haven't already (we have if this is a quadruple, etc)
                    if let Some(&last) = double_digits.last() {
                        if last == digit {
                            double_digits.pop();
                        }
                    }
                } else {
                    double_digits.push(digit);
                }
            }
        }
        if increasing && !double_digits.is_empty() {
            total += 1;
        }
    }

    println!("{}", total);
}

fn get_digits(d: u64, v: &mut Vec<u64>) -> () {
    if d > 10 {
        get_digits(d / 10, v);
    }
    v.push(d % 10);
}
