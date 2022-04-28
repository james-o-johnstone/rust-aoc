fn main() {
    let position = position(&get_movements("input".into()));
    println!("{}", position);
}

fn get_movements(filename: String) -> Vec<String> {
    std::fs::read_to_string(filename)
        .expect("file not found")
        .lines()
        .map(|x| x.parse::<String>().expect("invalid line"))
        .collect()
}

struct Submarine {
    x: i32,
    depth: i32,
    aim: i32
} 

impl Submarine {
    fn forward(&mut self, distance: i32) {
        self.x += distance; 
        self.depth += self.aim * distance;
    }

    fn up(&mut self, distance: i32) {
        self.aim -= distance;
    }

    fn down(&mut self, distance: i32) {
        self.aim += distance;
    }
}

impl Default for Submarine {
    fn default() -> Submarine {
        Submarine { x: 0, depth: 0, aim: 0 }
    }
}

fn position(steps: &Vec<String>) -> i32 {
    let mut sub = Submarine {..Default::default()};

    for step in steps {
        let parts: Vec<_> = step.split_whitespace().collect();
        let direction: String = parts[0].into();
        let distance: i32 = String::from(parts[1]).parse::<i32>().expect("invalid distance");
        match direction.as_str() {
            "forward" => sub.forward(distance),
            "up" => sub.up(distance),
            "down" => sub.down(distance),
            _ => panic!("invalid direction {}", direction)
        }
        println!("x {} depth {} aim {}", sub.x, sub.depth, sub.aim);
    }
    sub.x * sub.depth
}

/*
forward 5
down 5
forward 8
up 3
down 8
forward 2
*/

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_() {
        let steps = vec![
            String::from("forward 5"),
            String::from("down 5"),
            String::from("forward 8"),
            String::from("up 3"),
            String::from("down 8"),
            String::from("forward 2")
        ];
        assert_eq!(position(&steps), 900);
    }
}
