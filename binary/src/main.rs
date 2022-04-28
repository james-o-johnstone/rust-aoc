use std::{collections::HashMap};

fn main() {
    let report = get_report("diagnostic_report".into());
    let rates = rates(&report);
    let power = power_consumption(rates.gamma, rates.eps);
    println!("power: {}", power);
    let ls = life_support(&report);
    println!("life support: {}", ls);

}

fn get_report(filename: String) -> Vec<String> {
    std::fs::read_to_string(filename)
        .expect("file not found")
        .lines()
        .map(|x| x.parse::<String>().expect("invalid line"))
        .collect()
}

fn power_consumption(gamma: i32, eps: i32) -> i32 {
    gamma * eps
}

fn life_support(report: &Vec<String>) -> i32 {
    oxygen(report, 0) * c02(report, 0)
}

struct Rates {
    gamma: i32,
    eps: i32
}

fn rates(report: &Vec<String>) -> Rates {
    let mut counts = HashMap::new();
    for line in report {
        for (i, c) in line.chars().enumerate() {
            *counts.entry(i).or_insert(0) += char::to_digit(c, 10).expect("invalid char");
        }
    }
    let mut g = String::from("");
    let mut e = String::from("");
    let size = report.len() as f32;
    for i in 0..report.first().unwrap().chars().count() {
        let c = counts.get(&i).unwrap();
        if *c > (size / 2.0).ceil() as u32 {
            g.push_str("1");
            e.push_str("0");
        } else {
            g.push_str("0");
            e.push_str("1");
        }
    }
    Rates{
        gamma: i32::from_str_radix(&g, 2).unwrap(),
        eps: i32::from_str_radix(&e, 2).unwrap()
    }
}

/*
1 == "1"
0 == same number of "1" and "0"
-1 == "0"
*/
fn most_common(report: &Vec<String>, precedence: u32, nth_bit: u32) -> u32 {
    let mut sum = 0;
    for bits in report {
        if bits.chars().nth(nth_bit as usize).unwrap() == '1' {
            sum += 1;
        } else {
            sum -= 1;
        }
    }
    if sum > 0 {
        1
    } else if sum == 0 {
        precedence
    } else {
        0
    }
}

fn oxygen(report: &Vec<String>, nth_bit: u32) -> i32 {
    if report.len() == 1 {
        return i32::from_str_radix(report.first().unwrap(), 2).unwrap();
    }
    let most_common = most_common(report, 1, nth_bit);
    let mut next_bits = report
        .into_iter()
        .filter(
            |b| char::to_digit(b.chars().nth(nth_bit as usize).unwrap(), 10).unwrap() == most_common
        )
        .cloned()
        .collect::<Vec<_>>();
    return oxygen(&next_bits, nth_bit + 1);
}

fn c02(report: &Vec<String>, nth_bit: u32) -> i32 {
    if report.len() == 1 {
        return i32::from_str_radix(report.first().unwrap(), 2).unwrap();
    }
    let most_common = most_common(report, 1, nth_bit);
    let mut next_bits = report
        .into_iter()
        .filter(
            |b| char::to_digit(b.chars().nth(nth_bit as usize).unwrap(), 10).unwrap() != most_common
        )
        .cloned()
        .collect::<Vec<_>>();
    return c02(&next_bits, nth_bit + 1);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    /* diagnostic report
    00100
    11110
    10110
    10111
    10101
    01111
    00111
    11100
    10000
    11001
    00010
    01010

    gamma is most common bit in each position
    epsilon is least common bit in each position (!gamma)
    what if they are equal?
     */

    #[test]
    fn test_power() {
        assert_eq!(power_consumption(9, 22), 198);
    }

    #[test]
    fn test_rates() {
        let report = vec![
            String::from("00100"),
            String::from("11110"),
            String::from("10110"),
            String::from("10111"),
            String::from("10101"),
            String::from("01111"),
            String::from("00111"),
            String::from("11100"),
            String::from("10000"),
            String::from("11001"),
            String::from("00010"),
            String::from("01010"),
        ];
        assert_eq!(rates(&report).gamma, 22);
        assert_eq!(rates(&report).eps, 9);
    }

    #[test]
    fn test_oxygen() {
        let report = vec![
            String::from("00100"),
            String::from("11110"),
            String::from("10110"),
            String::from("10111"),
            String::from("10101"),
            String::from("01111"),
            String::from("00111"),
            String::from("11100"),
            String::from("10000"),
            String::from("11001"),
            String::from("00010"),
            String::from("01010"),
        ];
        assert_eq!(oxygen(&report, 0), 23);
    }

    #[test]
    fn test_c02() {
        let report = vec![
            String::from("00100"),
            String::from("11110"),
            String::from("10110"),
            String::from("10111"),
            String::from("10101"),
            String::from("01111"),
            String::from("00111"),
            String::from("11100"),
            String::from("10000"),
            String::from("11001"),
            String::from("00010"),
            String::from("01010"),
        ];
        assert_eq!(c02(&report, 0), 10);
    }
}
