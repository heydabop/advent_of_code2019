fn main() {
    let min = 172851;
    let max = 675869;

    let mut total = 0;

    for code in min..=max {
        let mut double = false;
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
            if !double && digit == last_digit {
                double = true;
            }
        }
        if increasing && double {
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
