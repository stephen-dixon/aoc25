use std::fs;
use std::error::Error;

fn main() ->  Result<(), Box<dyn Error>>
{
    println!("task 1");
    let input: String = fs::read_to_string("input.txt")?;

    let banks = parse_lines(&input);
    let max_vals = max_jolts(&banks);
    let total: u32 = max_vals.into_iter().sum();
    println!("total: {}", total);
    return Ok(())
}

fn parse_lines(lines: &str) -> Vec<Vec<u32>>
{
    let jolt_banks = lines.lines().map(|l| 
        l.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>()
    ).collect::<Vec<Vec<u32>>>();
    // println!("{:?}", jolt_banks);
    return jolt_banks;
}

fn max_jolts(banks: &Vec<Vec<u32>>) -> Vec<u32>
{
    let vals = banks.into_iter().map(|bank| {
        let l = bank.len();
        let max_val = bank[..(l-1)].iter().max().unwrap();
        let indices = bank.iter().enumerate()
            .filter_map(|(i, &x)| (x == *max_val).then_some(i))
            .collect::<Vec<usize>>();
        // let second_val = indices.into_iter().map(|i| {
        //     bank[(i+1)..]
        let second_val: u32 = *bank[(indices[0] +1)..].iter().max().unwrap();
        (max_val * 10) + second_val
        }).collect::<Vec<u32>>();
    return vals;
}

#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn test_parse_jolt_banks()
    {
        let lines = "987654321111111
811111111111119
234234234234278
818181911112111";
        let banks = parse_lines(&lines);
        assert!(banks == [[9,8,7,6,5,4,3,2,1,1,1,1,1,1,1],
            [8,1,1,1,1,1,1,1,1,1,1,1,1,1,9],
            [2,3,4,2,3,4,2,3,4,2,3,4,2,7,8],
            [8,1,8,1,8,1,9,1,1,1,1,2,1,1,1]]);
    }

    #[test]
    fn test_max_jolts()
    {
        let lines = "987654321111111
811111111111119
234234234234278
818181911112111";
        let banks = parse_lines(&lines);
        let vals = max_jolts(&banks);
        assert!(vals == [98, 89, 78, 92]);
    }
}
