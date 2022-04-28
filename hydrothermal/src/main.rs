use std::cmp::max;

#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(Clone)]
struct Point{
    x: usize,
    y: usize
}

#[derive(Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(Clone)]
struct Line{
    start: Point,
    end: Point
}

#[derive(Debug)]
struct Grid{
    grid: Vec<Vec<i32>>
}
impl Grid{
    pub fn new(extent: Point) -> Self {
        Grid { grid: vec![vec![0; extent.y]; extent.x] }
    }

    pub fn plot(&mut self, line: Line) {
        if line.start.x != line.end.x && line.start.y != line.end.y {
            self.plot_diag(line);
        } else {
            self.plot_straight(line)
        }
    }

    fn plot_diag(&mut self, line: Line) {
        // 1,1 -> 3,3 covers 1,1 2,2 3,3
        // 9,7 -> 7,9 covers 9,7 8,8 7,9

        // double iteration - for each movement in x also move in y
        // loop x1 from x1 -> x3, for each step go y1 from y1 -> y3
        // diag is always 45 deg so it has to 1 : 1 x:y
        
        /*
        x1=9, x2=7
        y1=7, y2=9
        */

        let x_iter = if line.start.x < line.end.x {
            (line.start.x ..= line.end.x).collect::<Vec<usize>>()
        } else {
            (line.end.x ..= line.start.x).rev().collect::<Vec<usize>>()
        };

        let y_iter = if line.start.y < line.end.y {
            (line.start.y ..= line.end.y).collect::<Vec<usize>>()
        } else {
            (line.end.y ..= line.start.y).rev().collect::<Vec<usize>>()
        };


        let diag_iter = Iterator::zip(
            x_iter.iter(),
            y_iter.iter()
        );

        let dist = ((line.end.x as f64 - line.start.x as f64).powf(2.0) + (line.end.y as f64 - line.start.y as f64).powf(2.0)).sqrt();
        let mut dist_count = 0;
        for (x,y ) in diag_iter {
            dist_count += 1;
            self.grid[*y][*x] += 1;
        }
    }
    
    pub fn plot_straight(&mut self, line: Line) {
        if line.start.x != line.end.x && line.start.y != line.end.y {
            return
        }
        if line.start.x == line.end.x {
            let mut range = [line.start.y, line.end.y];
            range.sort();
            for y in range[0] ..=range[1] {
                self.grid[y][line.start.x] += 1;
            }
        } else {
            let mut range = [line.start.x, line.end.x];
            range.sort();
            for x in range[0] ..=range[1] {
                self.grid[line.start.y][x] += 1;
            }
        };
    }

    pub fn count_dangerous(&self) -> i32 {
        self.grid
            .iter()
            .flatten()
            .cloned()
            .filter(|g| *g >= 2)
            .collect::<Vec<_>>()
            .len() as i32
    }

    pub fn print(&self) {
        for x in self.grid.iter() {
            for y in x.iter() {
                print!("{},", y);
            }
            println!();
        }
    }
}

fn parse_input(filename: String) -> Vec<Line> {
    std::fs::read_to_string(filename)
        .expect("file not found")
        .lines()
        .map(|l| {
            let pair = l.split(" -> ").collect::<Vec<&str>>();
            let start = pair[0]
                .split(",")
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();
            let end = pair[1]
                .split(",")
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            Line{
                start: Point {x: start[0] as usize, y: start[1] as usize},
                end: Point {x: end[0] as usize, y: end[1] as usize}
            }
        })
        .collect()
}

fn find_extent(lines: Vec<Line>) -> Point {
    let max_point = lines.into_iter().fold(0,|mut accum, l| {
        accum = max(
            max(
                max(l.start.x, l.start.y),
                max(l.end.x, l.end.y)
            ),
            accum
        );
        accum
    });
    Point {x: max_point+1, y: max_point+1}
}

fn main() {
    let lines = parse_input(String::from("input"));
    let extent = find_extent(lines.clone());
    let mut straight_grid = Grid::new(extent.clone());
    for line in lines.clone() {
        straight_grid.plot_straight(line);
    }
    println!(
        "part 1: {}",
        straight_grid.count_dangerous()
    );

    let mut diag_grid = Grid::new(extent.clone());
    for line in lines.clone() {
        diag_grid.plot(line);
    }
    println!(
        "part 2: {}",
        diag_grid.count_dangerous()
    );
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_plot() {
        let extent = Point{x: 10, y: 10};
        let mut grid = Grid::new(extent);
        grid.plot_straight(Line{
            start: Point{x:2, y: 2},
            end: Point{x:2, y: 1}
        });
        
        assert_eq!(
            grid.grid,
            vec![
                vec![0,0,0,0,0,0,0,0,0,0],
                vec![0,0,1,0,0,0,0,0,0,0],
                vec![0,0,1,0,0,0,0,0,0,0],
                vec![0,0,0,0,0,0,0,0,0,0],
                vec![0,0,0,0,0,0,0,0,0,0],
                vec![0,0,0,0,0,0,0,0,0,0],
                vec![0,0,0,0,0,0,0,0,0,0],
                vec![0,0,0,0,0,0,0,0,0,0],
                vec![0,0,0,0,0,0,0,0,0,0],
                vec![0,0,0,0,0,0,0,0,0,0],
            ]
        );

        //bottom row is formed by 0,9 -> 5,9 and 0,9 -> 2,9 overlapping
        grid.plot_straight(Line{
            start: Point{x:0, y: 9},
            end: Point{x:5, y: 9}
        });
        grid.plot_straight(Line{
            start: Point{x:0, y: 9},
            end: Point{x:2, y: 9}
        });

        assert_eq!(
            grid.grid,
            vec![
                vec![0,0,0,0,0,0,0,0,0,0],
                vec![0,0,1,0,0,0,0,0,0,0],
                vec![0,0,1,0,0,0,0,0,0,0],
                vec![0,0,0,0,0,0,0,0,0,0],
                vec![0,0,0,0,0,0,0,0,0,0],
                vec![0,0,0,0,0,0,0,0,0,0],
                vec![0,0,0,0,0,0,0,0,0,0],
                vec![0,0,0,0,0,0,0,0,0,0],
                vec![0,0,0,0,0,0,0,0,0,0],
                vec![2,2,2,1,1,1,0,0,0,0]
            ]
        );
    }

    #[test]
    fn test_count_dangerous() {
        let extent = Point{x: 10, y: 10};
        let mut grid = Grid::new(extent);

        grid.plot_straight(Line{
            start: Point{x:0, y: 9},
            end: Point{x:5, y: 9}
        });
        grid.plot_straight(Line{
            start: Point{x:8, y: 0},
            end: Point{x:0, y: 8}
        });
        grid.plot_straight(Line{
            start: Point{x:9, y: 4},
            end: Point{x:3, y: 4}
        });
        grid.plot_straight(Line{
            start: Point{x:2, y: 2},
            end: Point{x:2, y: 1}
        });
        grid.plot_straight(Line{
            start: Point{x:7, y: 0},
            end: Point{x:7, y: 4}
        });
        grid.plot_straight(Line{
            start: Point{x:6, y: 4},
            end: Point{x:2, y: 0}
        });
        grid.plot_straight(Line{
            start: Point{x:0, y: 9},
            end: Point{x:2, y: 9}
        });
        grid.plot_straight(Line{
            start: Point{x:3, y: 4},
            end: Point{x:1, y: 4}
        });
        grid.plot_straight(Line{
            start: Point{x:0, y: 0},
            end: Point{x:8, y: 8}
        });
        grid.plot_straight(Line{
            start: Point{x:5, y: 5},
            end: Point{x:8, y: 2}
        });
        assert_eq!(
            grid.grid,
            vec![
                vec![0,0,0,0,0,0,0,1,0,0],
                vec![0,0,1,0,0,0,0,1,0,0],
                vec![0,0,1,0,0,0,0,1,0,0],
                vec![0,0,0,0,0,0,0,1,0,0],
                vec![0,1,1,2,1,1,1,2,1,1],
                vec![0,0,0,0,0,0,0,0,0,0],
                vec![0,0,0,0,0,0,0,0,0,0],
                vec![0,0,0,0,0,0,0,0,0,0],
                vec![0,0,0,0,0,0,0,0,0,0],
                vec![2,2,2,1,1,1,0,0,0,0]
            ]
        );
        assert_eq!(grid.count_dangerous(), 5);
    }

    #[test]
    fn test_parse() {
        let extent = Point{x: 10, y: 10};
        let lines = parse_input(String::from("test_input"));
        let mut grid = Grid::new(extent);
        for line in lines {
            grid.plot_straight(line);
        }
        assert_eq!(grid.count_dangerous(), 5);
    }

    #[test]
    fn test_extent() {
        let lines = parse_input(String::from("test_input"));
        let extent = find_extent(lines);
        assert_eq!(extent, Point{x:9, y:9});
    }

    #[test]
    fn test_plot_diag() {
        let extent = Point{x: 10, y: 10};
        let mut grid = Grid::new(extent);
        grid.plot(Line{
            start: Point{x:0, y: 0},
            end: Point{x:9, y: 9}
        });
        grid.plot(Line{
            start: Point{x:0, y: 9},
            end: Point{x:9, y: 0}
        });
        grid.plot(Line{
            start: Point{x:9, y: 0},
            end: Point{x:0, y: 9}
        });
        grid.plot(Line{
            start: Point{x:0, y: 1},
            end: Point{x:8, y: 9}
        });
        grid.print();
        
        assert_eq!(
            grid.grid,
            vec![
                vec![1,0,0,0,0,0,0,0,0,2],
                vec![1,1,0,0,0,0,0,0,2,0],
                vec![0,1,1,0,0,0,0,2,0,0],
                vec![0,0,1,1,0,0,2,0,0,0],
                vec![0,0,0,1,1,2,0,0,0,0],
                vec![0,0,0,0,3,1,0,0,0,0],
                vec![0,0,0,2,0,1,1,0,0,0],
                vec![0,0,2,0,0,0,1,1,0,0],
                vec![0,2,0,0,0,0,0,1,1,0],
                vec![2,0,0,0,0,0,0,0,1,1],
            ]
        );
    }

    #[test]
    fn test_diag() {
        let lines = parse_input(String::from("test_input"));
        let extent = find_extent(lines.clone());
        let mut grid = Grid::new(extent);
        for line in lines {
            grid.plot(line);
        }
        assert_eq!(grid.count_dangerous(), 12);
    }
}
