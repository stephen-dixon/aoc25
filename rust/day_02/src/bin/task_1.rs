use std::fs;
use std::error::Error;

fn main() ->  Result<(), Box<dyn Error>>
{
    println!("task 1");
    let input: String = fs::read_to_string("input.txt")?;

    let id_ranges = parse_lines(&input);
    let invalid_ids = find_invalid_ids(&id_ranges);
    let total: usize = invalid_ids.into_iter().map(|x| x as usize).sum();
    println!("total: {}", total);
    return Ok(())
}

fn parse_lines(lines: &str) -> Vec<(&str, &str)>
{
    let ranges = lines.split(',')
        .map(|s| s.split_once('-').unwrap())
        .collect::<Vec<(&str, &str)>>();
    // println!("{:?}", ranges);
    return ranges;
}

fn is_invalid(id_int: u64) -> bool
{
    let id = id_int.to_string();
    let len = id.len();
    if len % 2 > 0 {
        return false;
    }
    let (left, right) = id.split_at(len / 2);
    return left == right;
}

fn find_invalid_ids(id_ranges: &Vec<(&str, &str)>) -> Vec<u64>
{
    let vals = id_ranges.into_iter()
        .flat_map( |(start_s, stop_s)| {
            let start = start_s.trim().parse::<u64>().unwrap_or_else(|_| {
                panic!("can't convert val to int {}", start_s);});
            let stop = stop_s.trim().parse::<u64>().unwrap_or_else(|_| {
                panic!("can't convert val to int {}", stop_s);});
            (start..=stop).filter(|&x| is_invalid(x))
                .collect::<Vec<u64>>()
        }).collect::<Vec<u64>>();
    return vals;

}

#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn test_parse_to_pairs()
    {
        let lines = "11-22,95-115,998-1012,1188511880-1188511890";
        let pairs = parse_lines(&lines);
        assert!(pairs == [("11","22"), ("95","115"), ("998","1012"),("1188511880","1188511890")]);
    }

    #[test]
    fn test_search()
    {
        let lines = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let inputs = parse_lines(&lines);
        let ids = find_invalid_ids(&inputs);
        assert!(ids == [11,22,99,1010,1188511885,222222,446446,38593859]);
    }

}
