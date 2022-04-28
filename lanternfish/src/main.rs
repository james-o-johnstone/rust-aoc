use std::{collections::HashMap, hash::Hash};

fn main() {
    let input = vec![3,3,2,1,4,1,1,2,3,1,1,2,1,2,1,1,1,1,1,1,4,1,1,5,2,1,1,2,1,1,1,3,5,1,5,5,1,1,1,1,3,1,1,3,2,1,1,1,1,1,1,4,1,1,1,1,1,1,1,4,1,3,3,1,1,3,1,3,1,2,1,3,1,1,4,1,2,4,4,5,1,1,1,1,1,1,4,1,5,1,1,5,1,1,3,3,1,3,2,5,2,4,1,4,1,2,4,5,1,1,5,1,1,1,4,1,1,5,2,1,1,5,1,1,1,5,1,1,1,1,1,3,1,5,3,2,1,1,2,2,1,2,1,1,5,1,1,4,5,1,4,3,1,1,1,1,1,1,5,1,1,1,5,2,1,1,1,5,1,1,1,4,4,2,1,1,1,1,1,1,1,3,1,1,4,4,1,4,1,1,5,3,1,1,1,5,2,2,4,2,1,1,3,1,5,5,1,1,1,4,1,5,1,1,1,4,3,3,3,1,3,1,5,1,4,2,1,1,5,1,1,1,5,5,1,1,2,1,1,1,3,1,1,1,2,3,1,2,2,3,1,3,1,1,4,1,1,2,1,1,1,1,3,5,1,1,2,1,1,1,4,1,1,1,1,1,2,4,1,1,5,3,1,1,1,2,2,2,1,5,1,3,5,3,1,1,4,1,1,4];
    println!("{}", simulate(input, 256));
}

fn seed(input: Vec<i64>) -> HashMap<i64, i64> {
    let mut buckets = HashMap::new();

    for day in 0..=8 {
        buckets.insert(day, 0);
    }

    input.into_iter().for_each(|i| {
        *buckets.entry(i).or_insert(0) += 1;
    });

    buckets
}

fn day(buckets: &HashMap<i64, i64>) -> HashMap<i64, i64> {
    buckets.into_iter().fold(HashMap::new(), |mut next_buckets, (day, count)| {
        if day-1 < 0 {
            // create the newly spawned fish
            *next_buckets.entry(8).or_insert(0) += *count;
            // reset fish to max age
            *next_buckets.entry(6).or_insert(0) += *count;
        } else {
            // otherwise age by 1 day
            *next_buckets.entry(day-1).or_insert(0) += *count;
        };
        next_buckets
    })
}

fn simulate(input: Vec<i64>, days: i64) -> i64 {
    let mut total = 0;

    let mut buckets = seed(input);
    
    for d in 1..=days {
        buckets = day(&buckets);
    }
    buckets.values().sum()
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

/*
Initial state: 3,4,3,1,2
After  1 day:  2,3,2,0,1
After  2 days: 1,2,1,6,0,8
After  3 days: 0,1,0,5,6,7,8
After  4 days: 6,0,6,4,5,6,7,8,8
After  5 days: 5,6,5,3,4,5,6,7,7,8
After  6 days: 4,5,4,2,3,4,5,6,6,7
After  7 days: 3,4,3,1,2,3,4,5,5,6
After  8 days: 2,3,2,0,1,2,3,4,4,5
After  9 days: 1,2,1,6,0,1,2,3,3,4,8
After 10 days: 0,1,0,5,6,0,1,2,2,3,7,8
After 11 days: 6,0,6,4,5,6,0,1,1,2,6,7,8,8,8
After 12 days: 5,6,5,3,4,5,6,0,0,1,5,6,7,7,7,8,8
After 13 days: 4,5,4,2,3,4,5,6,6,0,4,5,6,6,6,7,7,8,8
After 14 days: 3,4,3,1,2,3,4,5,5,6,3,4,5,5,5,6,6,7,7,8
After 15 days: 2,3,2,0,1,2,3,4,4,5,2,3,4,4,4,5,5,6,6,7
After 16 days: 1,2,1,6,0,1,2,3,3,4,1,2,3,3,3,4,4,5,5,6,8
After 17 days: 0,1,0,5,6,0,1,2,2,3,0,1,2,2,2,3,3,4,4,5,7,8
After 18 days: 6,0,6,4,5,6,0,1,1,2,6,0,1,1,1,2,2,3,3,4,6,7,8,8,8,8

 */
    #[test]
    fn test_seed() {
        let mut buckets = seed(vec![3,4,3,1,2]);
        assert_eq!(
            buckets,
            HashMap::from([
                (0, 0),
                (1, 1),
                (2, 1),
                (3, 2),
                (4, 1),
                (5, 0),
                (6, 0),
                (7, 0),
                (8, 0)
            ])
        );

    }
    #[test]
    fn test_day() {
        let mut buckets = seed(vec![3,4,3,1,2]);

        buckets = day(&buckets);
        assert_eq!(
            buckets,
            HashMap::from([
                (0, 1),
                (1, 1),
                (2, 2),
                (3, 1),
                (4, 0),
                (5, 0),
                (6, 0),
                (7, 0),
                (8, 0)
            ])
        );
        buckets = day(&buckets);
        assert_eq!(
            buckets,
            HashMap::from([
                (0, 1),
                (1, 2),
                (2, 1),
                (3, 0),
                (4, 0),
                (5, 0),
                (6, 1),
                (7, 0),
                (8, 1)
            ])
        );
    }

    #[test]
    fn test_18_days() {
        assert_eq!(
            simulate(vec![3,4,3,1,2], 18),
            26
        );
    }

    #[test]
    fn test_80_days() {
        assert_eq!(
            simulate(vec![3,4,3,1,2], 80),
            5934
        );
    }

    #[test]
    fn test_256_days() {
        assert_eq!(
            simulate(vec![3,4,3,1,2], 256),
            26984457539
        );
    }
}
