fn main() {
    let depths = get_depths(String::from("input"));
    println!("sonar {}", sonar(&depths));
    println!("sliding window {}", sliding_window(&depths));
}

fn sonar(depths: &Vec<i32>) -> i32 {
    let mut count = 0i32;
    let mut last = depths.first().expect("depths is empty");
    for i in &depths[1..] {
        if i > last {
            count+=1;
        }
        last = i;
    }
    count
}

fn sliding_window(depths: &Vec<i32>) -> i32 {
    let mut i = 0;
    let mut count = 0;
    let mut last = 0;
    while i < depths.len() && i+3 <= depths.len() {
        let sum: i32 = depths[i..i+3].iter().sum();
        if i == 0 {
            last = sum;
            i += 1;
            continue
        }
        if sum > last {
            count += 1;
        }
        last = sum;
        i += 1;
    }
    count
}

fn get_depths(filename: String) -> Vec<i32> {
    std::fs::read_to_string(filename)
        .expect("file not found")
        .lines()
        .map(|x| x.parse::<i32>().expect("invalid line"))
        .collect()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_sonar() {
        let depths = vec![
            199,
            200,
            208,
            210,
            200,
            207,
            240,
            269,
            260,
            263
        ];
        assert_eq!(sonar(&depths), 7);
    }

/*
199  A      
200  A B    
208  A B C  
210    B C D
200  E   C D
207  E F   D
240  E F G  
269    F G H
260      G H
263        H
*/
    #[test]
    fn test_sliding_window() {
        let depths = vec![
            199,
            200,
            208,
            210,
            200,
            207,
            240,
            269,
            260,
            263
        ];
        assert_eq!(sliding_window(&depths), 5);
    }
}
