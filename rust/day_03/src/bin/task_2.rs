use std::fs;
use std::error::Error;

fn main() ->  Result<(), Box<dyn Error>>
{
    println!("task 2");
    let input: String = fs::read_to_string("input.txt")?;

    let banks = parse_lines(&input);
    let max_vals = max_jolts(&banks, 12);
    let total: u64 = max_vals.into_iter().sum();
    println!("total: {}", total);
    return Ok(())
}

fn parse_lines(lines: &str) -> Vec<&str>
{
    lines.lines().collect()
}

fn max_jolts_in_bank(bank: &[char], n: usize) -> Vec<char>
{
    let l = bank.len();
    let max: char = *bank[..(l-n+1)].iter().max().unwrap();
    if n == 1 {
        return vec!(max);
    }
    let index = bank.iter().position(|&x| x==max).unwrap();
    let mut result = max_jolts_in_bank(&bank[(index+1)..], n-1);
    result.push(max);
    return result;
    
}

fn max_jolts(banks: &Vec<&str>, n: usize) -> Vec<u64>
{
    let vals = banks.into_iter().map(|bank| {
        // let max = max_jolts_in_bank(&bank.chars().collect::<Vec<char>>(), n);
        let bank_chars = bank.chars().collect::<Vec<char>>();
        let mut vals: Vec<char> = max_jolts_in_bank(&bank_chars[..], n);
        vals.reverse();
        vals.iter().fold(0, |acc, c| 
            (acc*10) + ((*c as u8 - b'0') as u64)
        )
    }).collect::<Vec<u64>>();
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
        assert!(banks == ["987654321111111", "811111111111119", "234234234234278", "818181911112111"]);
    }

    #[test]
    fn test_max_jolts()
    {
        let lines = "987654321111111
811111111111119
234234234234278
818181911112111";
        let banks = parse_lines(&lines);
        let vals_2 = max_jolts(&banks, 2);
        assert!(vals_2 == [98, 89, 78, 92]);

        let vals_12 = max_jolts(&banks, 12);
        assert!(vals_12 == [987654321111,
            811111111119,
            434234234278,
            888911112111]);
    }
}
