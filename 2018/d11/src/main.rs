use std::cmp::max;

pub struct Grid {
    store: [[i32; 300]; 300],
}

impl Grid {
    pub fn new(serial: i32) -> Self {
        let mut store = [[0; 300]; 300];

        for (x, row) in store.iter_mut().enumerate() {
            for (y, col) in row.iter_mut().enumerate() {
                let rack = x as i32 + 10;
                let power = rack * y as i32;
                let power = power + serial;
                let power = power * rack;
                let nth = max(0, power.to_string().len() as i32 - 3) as usize;
                let hundreds = power
                    .to_string()
                    .chars()
                    .nth(nth)
                    .map_or(0, |c: char| c as u8 - '0' as u8);
                *col = hundreds as i32 - 5;
            }
        }

        Grid { store }
    }

    pub fn get(&self, x: usize, y: usize) -> i32 {
        self.store[x][y]
    }

    pub fn display(&self, x: usize, y: usize, size: usize) {
        for xx in x..x + size {
            for yy in 0..y + size {
                print!("|{},{} = {}| ", xx, yy, self.get(xx, yy));
            }
            println!("");
        }
    }
}

fn sub_search(grid: &Grid, x: usize, y: usize, s: usize) -> (usize, usize, i32) {
    let mut acc = 0;

    for xx in x..x + s {
        for yy in y..y + s {
            acc += grid.get(xx, yy);
        }
    }

    (x, y, acc)
}

fn search(grid: &Grid) -> (usize, usize, usize, i32) {
    let mut max = (0, 0, 0, 0);

    for size in 1..300 {
        for x in 0..300 {
            for y in 0..300 {
                if x + size < 300 && y + size < 300 {
                    let (xx, yy, power) = sub_search(grid, x, y, size);

                    if power > max.3 {
                        max = (xx, yy, size, power);
                        println!("max {:?}", max);
                    }
                }
            }
        }
    }

    max
}

fn main() {
    let g = Grid::new(1723);

    println!("search : {:?}", search(&g));
}

mod test {
    use crate::*;

    #[test]
    fn test_create_grid_0() {
        let g = Grid::new(8);

        assert_eq!(g.get(3, 5), 4);
    }

    #[test]
    fn test_create_grid_1() {
        let g = Grid::new(57);

        assert_eq!(g.get(122, 79), -5);
    }

    #[test]
    fn test_create_grid_2() {
        let g = Grid::new(39);

        assert_eq!(g.get(217, 196), 0);
    }

    #[test]
    fn test_create_grid_3() {
        let g = Grid::new(71);

        assert_eq!(g.get(101, 153), 4);
    }

    #[test]
    fn test_search_0() {
        let g = Grid::new(18);

        assert_eq!(search(&g), (90, 269, 16, 113));
    }

    #[test]
    fn test_search_1() {
        let g = Grid::new(42);

        assert_eq!(search(&g), (232, 251, 12, 119));
    }
}
