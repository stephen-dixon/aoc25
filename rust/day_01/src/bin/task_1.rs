use std::fs;
use std::error::Error;

fn main() ->  Result<(), Box<dyn Error>>
{
    println!("task 1");
    let input: String = fs::read_to_string("input.txt")?;

    let instructions = parse_lines(&input);
    let positions = execute_instructions(50, &instructions);
    let total = sum_occurences(0, &positions);
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
}
