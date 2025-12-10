use std::ops::Index;

/// Direction enum for cardinal directions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

/// Cardinal directions array
pub const CARDINAL: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::East,
    Direction::West,
];

/// Diagonal direction offsets (dx, dy)
pub const DIAGONAL: [(isize, isize); 4] = [(-1, -1), (-1, 1), (1, -1), (1, 1)];

/// All 8 directions (cardinal + diagonal)
pub const ALL_8: [(isize, isize); 8] = [
    (0, -1),  // North
    (0, 1),   // South
    (1, 0),   // East
    (-1, 0),  // West
    (-1, -1), // NW
    (-1, 1),  // SW
    (1, -1),  // NE
    (1, 1),   // SE
];

impl Direction {
    /// Returns the (dx, dy) offset for this direction
    pub fn offset(&self) -> (isize, isize) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
        }
    }
}

/// A 2D grid structure
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    /// Creates a new grid filled with default values
    pub fn new(width: usize, height: usize, default: T) -> Self
    where
        T: Clone,
    {
        Grid {
            data: vec![default; width * height],
            width,
            height,
        }
    }

    /// Creates a grid from a flat vector
    pub fn from_vec(data: Vec<T>, width: usize) -> Self {
        let height = data.len() / width;
        assert_eq!(
            data.len(),
            width * height,
            "Data length must be width * height"
        );
        Grid {
            data,
            width,
            height,
        }
    }

    /// Parses a grid from string lines using a converter function
    pub fn parse<F>(lines: &[String], f: F) -> Self
    where
        F: Fn(char) -> T,
    {
        if lines.is_empty() {
            return Grid {
                data: vec![],
                width: 0,
                height: 0,
            };
        }

        let height = lines.len();
        let width = lines[0].len();
        let mut data = Vec::with_capacity(width * height);

        for line in lines {
            for ch in line.chars() {
                data.push(f(ch));
            }
        }

        Grid {
            data,
            width,
            height,
        }
    }

    /// Gets a reference to the value at (x, y) if in bounds
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x < self.width && y < self.height {
            Some(&self.data[y * self.width + x])
        } else {
            None
        }
    }

    /// Gets a mutable reference to the value at (x, y) if in bounds
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if x < self.width && y < self.height {
            Some(&mut self.data[y * self.width + x])
        } else {
            None
        }
    }

    /// Checks if coordinates are within bounds
    pub fn contains(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    /// Returns the width of the grid
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the grid
    pub fn height(&self) -> usize {
        self.height
    }

    /// Returns an iterator over all cells with their coordinates
    pub fn iter(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        self.data.iter().enumerate().map(move |(i, v)| {
            let x = i % self.width;
            let y = i / self.width;
            (x, y, v)
        })
    }

    /// Returns the 4 cardinal neighbors of (x, y)
    pub fn neighbors4(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize, &T)> {
        let mut neighbors = Vec::new();

        for dir in &CARDINAL {
            if let Some((nx, ny, val)) = self.neighbor(x, y, *dir) {
                neighbors.push((nx, ny, val));
            }
        }

        neighbors.into_iter()
    }

    /// Returns all 8 neighbors (cardinal + diagonal) of (x, y)
    pub fn neighbors8(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize, &T)> {
        let mut neighbors = Vec::new();

        for (dx, dy) in &ALL_8 {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && ny >= 0 {
                let nx = nx as usize;
                let ny = ny as usize;
                if let Some(val) = self.get(nx, ny) {
                    neighbors.push((nx, ny, val));
                }
            }
        }

        neighbors.into_iter()
    }

    /// Returns the neighbor in a specific direction
    pub fn neighbor(&self, x: usize, y: usize, dir: Direction) -> Option<(usize, usize, &T)> {
        let (dx, dy) = dir.offset();
        let nx = x as isize + dx;
        let ny = y as isize + dy;

        if nx >= 0 && ny >= 0 {
            let nx = nx as usize;
            let ny = ny as usize;
            self.get(nx, ny).map(|v| (nx, ny, v))
        } else {
            None
        }
    }
}

impl Grid<char> {
    /// Parses a character grid
    pub fn parse_chars(lines: &[String]) -> Grid<char> {
        Grid::parse(lines, |c| c)
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        assert!(x < self.width && y < self.height, "Index out of bounds");
        &self.data[y * self.width + x]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_new() {
        let grid = Grid::new(3, 2, 0);
        assert_eq!(grid.width(), 3);
        assert_eq!(grid.height(), 2);
        assert_eq!(grid.get(0, 0), Some(&0));
        assert_eq!(grid.get(2, 1), Some(&0));
    }

    #[test]
    fn test_grid_from_vec() {
        let data = vec![1, 2, 3, 4, 5, 6];
        let grid = Grid::from_vec(data, 3);
        assert_eq!(grid.width(), 3);
        assert_eq!(grid.height(), 2);
        assert_eq!(grid.get(0, 0), Some(&1));
        assert_eq!(grid.get(2, 1), Some(&6));
    }

    #[test]
    fn test_grid_parse_chars() {
        let lines = vec!["abc".to_string(), "def".to_string()];
        let grid = Grid::parse_chars(&lines);
        assert_eq!(grid.width(), 3);
        assert_eq!(grid.height(), 2);
        assert_eq!(grid.get(0, 0), Some(&'a'));
        assert_eq!(grid.get(2, 1), Some(&'f'));
    }

    #[test]
    fn test_grid_get_out_of_bounds() {
        let grid = Grid::new(3, 2, 0);
        assert_eq!(grid.get(3, 0), None);
        assert_eq!(grid.get(0, 2), None);
    }

    #[test]
    fn test_grid_contains() {
        let grid = Grid::new(3, 2, 0);
        assert!(grid.contains(0, 0));
        assert!(grid.contains(2, 1));
        assert!(!grid.contains(3, 0));
        assert!(!grid.contains(0, 2));
    }

    #[test]
    fn test_grid_index() {
        let grid = Grid::from_vec(vec![1, 2, 3, 4], 2);
        assert_eq!(grid[(0, 0)], 1);
        assert_eq!(grid[(1, 1)], 4);
    }

    #[test]
    fn test_grid_neighbors4() {
        let grid = Grid::from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 3);
        let neighbors: Vec<_> = grid.neighbors4(1, 1).collect();
        assert_eq!(neighbors.len(), 4);
        // Should have neighbors: (1, 0)=2, (1, 2)=8, (2, 1)=6, (0, 1)=4
        let values: Vec<_> = neighbors.iter().map(|(_, _, v)| **v).collect();
        assert!(values.contains(&2));
        assert!(values.contains(&8));
        assert!(values.contains(&6));
        assert!(values.contains(&4));
    }

    #[test]
    fn test_grid_neighbors4_corner() {
        let grid = Grid::from_vec(vec![1, 2, 3, 4], 2);
        let neighbors: Vec<_> = grid.neighbors4(0, 0).collect();
        assert_eq!(neighbors.len(), 2); // Only right and down
    }

    #[test]
    fn test_grid_neighbors8() {
        let grid = Grid::from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 3);
        let neighbors: Vec<_> = grid.neighbors8(1, 1).collect();
        assert_eq!(neighbors.len(), 8); // All 8 neighbors
    }

    #[test]
    fn test_direction_offset() {
        assert_eq!(Direction::North.offset(), (0, -1));
        assert_eq!(Direction::South.offset(), (0, 1));
        assert_eq!(Direction::East.offset(), (1, 0));
        assert_eq!(Direction::West.offset(), (-1, 0));
    }

    #[test]
    fn test_grid_neighbor() {
        let grid = Grid::from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 3);
        let north = grid.neighbor(1, 1, Direction::North);
        assert_eq!(north, Some((1, 0, &2)));

        let south = grid.neighbor(1, 1, Direction::South);
        assert_eq!(south, Some((1, 2, &8)));

        // Edge case: no neighbor
        let north_edge = grid.neighbor(1, 0, Direction::North);
        assert_eq!(north_edge, None);
    }

    #[test]
    fn test_grid_iter() {
        let grid = Grid::from_vec(vec![1, 2, 3, 4], 2);
        let items: Vec<_> = grid.iter().collect();
        assert_eq!(items.len(), 4);
        assert_eq!(items[0], (0, 0, &1));
        assert_eq!(items[3], (1, 1, &4));
    }
}
