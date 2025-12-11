use std::fs;
use std::error::Error;
use itertools::Itertools;
// use std::collections::HashSet;

fn main() ->  Result<(), Box<dyn Error>>
{
    println!("task 1");
    let input: String = fs::read_to_string("input.txt")?;

    let grid = Board::new(&input.lines().collect::<Vec<&str>>());
    let total = grid.count_neighbours().into_iter()
        .filter(|(_, val)| *val < 4)
        .count();

    println!("total: {}", total);
    return Ok(())
}

#[derive(Debug)]
enum Tile {
    Roll,
    Space,
}

#[derive(Debug)]
struct Board {
    tiles: Vec<Tile>,
    rows: usize,
    cols: usize,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Board {
    fn new(lines: &Vec<&str>) -> Board 
    {
        let rows = lines.len();
        let cols = lines[0].len();
        let tiles = lines.concat().chars().map(|c| {
            match c {
                '@' => Tile::Roll,
                '.' => Tile::Space,
                _ => panic!("unrecognised symbol in input"),
            }
        }).collect::<Vec<Tile>>();
        return Board { tiles: tiles, rows: rows, cols: cols};
    }

    fn index(&self, position: Point) -> usize
    {
        return (position.y * self.cols) + position.x
    }

    fn position(&self, i: usize) -> Point
    {
        let x = i % self.cols as usize;
        let y = (i - x) / self.cols;
        return Point{x: x, y: y}
    }

    fn neighbours(&self, position: Point) -> Vec<Point>
    {
        let neighbours = ((position.x as i64 - 1)..(position.x as i64 + 2))
            .cartesian_product((position.y as i64 - 1)..(position.y as i64 + 2))
            .filter(|(x, y)| 
                *x >= 0 && *y >= 0 && 
                !(*x as usize == position.x && *y as usize== position.y) &&
                (*x as usize) < self.cols && (*y as usize) < self.rows
                )
            .map(|(x, y)| {
                Point{x:x as usize, y:y as usize}
        }).collect::<Vec<Point>>();
        return neighbours;
    }

    fn count_neighbours(&self) -> Vec<(usize, usize)>
    {
        let counts = self.tiles.iter()
            .enumerate()
            .filter(|(_, t)| matches!(*t, Tile::Roll))
            .map(|(i, _)| {
            let p = self.position(i);
            let neighbours = self.neighbours(p);
            let count = neighbours.iter().map(|p| self.index(*p))
                .filter(|idx| matches!(self.tiles[*idx], Tile::Roll))
                .count();
            (i, count)
        }).collect::<Vec<(usize, usize)>>();
        // println!("{:?}", counts);
        return counts;
    }
}

#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn test_parse_input()
    {
        let lines = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let grid = Board::new(&lines.lines().collect::<Vec<&str>>());
        assert!(grid.cols == 10);
        assert!(grid.rows == 10);
        assert!(matches!(grid.tiles[0], Tile::Space));
        assert!(matches!(grid.tiles[10], Tile::Roll));
    }

        #[test]
    fn test_get_neighbourss()
    {
        let lines = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let grid = Board::new(&lines.lines().collect::<Vec<&str>>());
        let n = grid.neighbours(Point{x:0, y:0}).into_iter().collect::<HashSet<_>>();
        // println!("{:?}", n);
        assert!(n
            == [Point{x:1, y:0}, Point{x:0, y:1}, Point{x:1, y:1}].into_iter().collect::<HashSet<_>>());
    }

    #[test]
    fn test_count_neighbours()
    {
        let lines = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let grid = Board::new(&lines.lines().collect::<Vec<&str>>());
        let counts: Vec<(usize, usize)> = grid.count_neighbours().into_iter()
            .filter(|(_, val)| *val < 4)
            .collect();
        // println!("{:?}", counts);
        let accessible = counts.into_iter().count();
        // println!("count: {}", accessible);
        assert!(accessible == 13);
    }
}
