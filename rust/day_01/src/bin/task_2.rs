use std::fs;
use std::error::Error;

fn main() ->  Result<(), Box<dyn Error>>
{
    println!("task 2");
    let input: String = fs::read_to_string("input.txt")?;

    let instructions = parse_lines(&input);
    let hits = spin(50, 0, &instructions);
    let total: u32 = hits.into_iter().sum();
    println!("total: {}", total);
    return Ok(())
}

fn parse_lines(lines: &str) -> Vec<(char, u32)>
{
    let pairs = lines.lines()
        .map( |l|
            { let mut chars = l.chars();
                let letter = chars.next().unwrap();
                let value: u32 = chars.as_str().parse::<u32>().unwrap();
                (letter, value)
            }).collect::<Vec<(char, u32)>>();
    // print!("{:?}", pairs);
    return pairs;
}

fn execute_instructions(start: u32, instructions: &Vec<(char, u32)>) -> Vec<u32>
{
    let mut position: i32 = start as i32;
    let positions = instructions.into_iter().map(|(direction, val)| {
        match direction {
            'L' => {position = (((position - *val as i32) % 100) + 100) % 100;},
            'R' => {position = (position + *val as i32 ) % 100;},
            _ => panic!("unknown direction specified")
        }
        // println!("{}", position);
        u32::try_from(position).unwrap()
    }).collect::<Vec<u32>>();
    // print!("{:?}", positions);
    return positions;
}

fn spin(start: u32, target: u32, instructions: &Vec<(char, u32)>) -> Vec<u32>
{
    let mut position: i32 = start as i32;
    let hits = instructions.into_iter().map(|(direction, val)| {
        let mut acc: u32 = val / 100;
        // print!("{} ", acc);
        match direction {
            'L' => {
                if position != 0 && *val as i32 % 100 > position {
                    acc += 1;
                }
                position = (((position - *val as i32) % 100) + 100) % 100;
                if position == target as i32 {
                    acc += 1;
                }
            },
            'R' => {
                let new = (position + *val as i32 ) % 100;
                if new < position {
                    acc += 1;
                }
                position = new;
            },
            _ => panic!("unknown direction specified")
        }
        // println!("dir: {}, position: {}, hits: {}", direction, position, acc);
        acc
    }).collect::<Vec<u32>>();
    return hits;
}

fn sum_occurences(val: u32, positions: &Vec<u32>) -> usize
{
    positions.iter().filter(|&&x| x == val).count()
}

#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn test_parse_to_pairs()
    {
        let lines = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        let pairs = parse_lines(&lines);
        assert!(pairs == [('L',68), ('L',30), ('R',48), ('L',5), ('R',60), ('L',55),
        ('L',1), ('L',99), ('R',14), ('L',82)]);
    }

    #[test]
    fn test_execute_instructions()
    {
        let instructions: Vec<(char, u32)> = [('L',68), ('L',30), ('R',48), ('L',5), ('R',60), ('L',55), ('L',1), ('L',99), ('R',14), ('L',82)].to_vec();
        let positions = execute_instructions(50, &instructions);
        assert!(positions == [82,52,0,95,55,0,99,0,14,32]);
    }

    #[test]
    fn test_count()
    {
        let positions = [82,52,0,95,55,0,99,0,14,32].to_vec();
        let count = sum_occurences(0, &positions);
        assert!(count == 3);
    }

    #[test]
    fn test_spin()
    {
        let instructions: Vec<(char, u32)> = [('R',1000), ('R',200), ('L', 500), ('L',68), ('L',30), ('R',48), ('L',5), ('R',60), ('L',55), ('L',1), ('L',99), ('R',14), ('L',82)].to_vec();
        let hits = spin(50, 0, &instructions);
        assert!(hits == [10,2,5,1,0,1,0,1,1,0,1,0,1]);
    }

    #[test]
    fn test_spin_and_count()
    {
        let instructions: Vec<(char, u32)> = [('L',68), ('L',30), ('R',48), ('L',5), ('R',60), ('L',55), ('L',1), ('L',99), ('R',14), ('L',82)].to_vec();
        let hits = spin(50, 0, &instructions);
        assert!(hits == [1,0,1,0,1,1,0,1,0,1]);
        let count: u32 = hits.into_iter().sum();
        assert!(count == 6);
    }
}
