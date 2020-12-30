use std::fs;

#[derive(PartialEq)]
enum Seat {
    Chair(bool),
    NotChair,
}

struct SeatGrid {
    grid: Vec<Vec<Seat>>,
    occupied: usize,
}

impl SeatGrid {
    pub fn new(grid: Vec<Vec<Seat>>) -> Self {
        let occupied = grid.iter().flatten().fold(0, |acc, val| if *val == Seat::Chair(true) { acc + 1 } else { acc });
        Self {
            grid,
            occupied,
        }
    }

    pub fn from_str(s: &str) -> Self {
    let grid = s
        .split('\n')
        .filter(|s| s.len() > 0)
        .map(|s| s
             .chars()
             .map(|c| match c {
                 '.' => Seat::NotChair,
                 'L' => Seat::Chair(false),
                 '#' => Seat::Chair(true),
                 n => { println!("{}", n); panic!(); }
             })
             .collect::<Vec<_>>()
        )
        .collect::<Vec<_>>();

        let occupied = grid.iter().flatten().fold(0, |acc, val| if *val == Seat::Chair(true) { acc + 1 } else { acc });

        Self {
            grid,
            occupied,
        }
    }

    fn len(&self) -> (usize, usize) {
        (self.grid.len(), self.grid[0].len())
    }

    fn get_state(&self, row: usize, col: usize) -> &Seat {
        &self.grid[row][col]
    }

    fn is_occupied(&self, row: usize, col: usize) -> bool {
        match self.grid[row][col] {
            Seat::Chair(x) => x,
            _ => false
        }
    }

    pub fn next_generation(&mut self) -> bool {
        let (row_len, col_len) = self.len();
        let mut new_grid: Vec<Vec<Seat>> = Vec::with_capacity(row_len);
        let mut has_changed = false;

        for i in 0..row_len {
            let mut new_row: Vec<Seat> = Vec::with_capacity(col_len);
            for j in 0..col_len {
                let state = self.get_state(i, j);
                let mut occupied_neighbours = 0;

                for k in -1i8..=1 {
                    for l in -1i8..=1 {
                        if k == 0 && l == 0 {
                            continue;
                        }

                        let row = i as i8 + k;
                        let col = j as i8 + l;

                        if row >= 0 && col >= 0 && row < row_len as i8 && col < col_len as i8 {
                            if self.is_occupied(row as usize, col as usize) {
                                occupied_neighbours += 1;
                            }
                        }
                    }
                }

                let seat = match state {
                    Seat::Chair(x) => {
                        if *x && occupied_neighbours >= 4 {
                            has_changed = true;
                            Seat::Chair(false)
                        } else if !x && occupied_neighbours == 0 {
                            has_changed = true;
                            Seat::Chair(true)
                        } else {
                            Seat::Chair(*x)
                        }
                    },
                    Seat::NotChair => Seat::NotChair,
                };
                new_row.push(seat);
            }
            new_grid.push(new_row);
        }

        self.occupied = new_grid.iter().flatten().fold(0, |acc, val| if *val == Seat::Chair(true) { acc + 1 } else { acc });
        self.grid = new_grid;

        has_changed
    }
}

fn read_file() -> Result<SeatGrid, Box<dyn std::error::Error + 'static>> {
    let data_str = fs::read_to_string("assets/input11.txt")?;
    Ok(SeatGrid::from_str(&data_str))
}


pub fn find() -> Result<String, Box<dyn std::error::Error + 'static>> {
    let mut data = read_file()?;

    loop {
        if !data.next_generation() { break; }
    }

    Ok(data.occupied.to_string())
}
