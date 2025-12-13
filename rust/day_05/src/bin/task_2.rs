use std::fs;
use std::error::Error;
use std::ops::Range;
use std::cmp::max;
use intervaltree::IntervalTree;

fn main() ->  Result<(), Box<dyn Error>>
{
    println!("task 2");
    let input: String = fs::read_to_string("input.txt")?;

    let (_, ranges) = parse_input(&input);
    let total = count_fresh_items(&ranges);

    println!("total: {}", total);
    return Ok(())
}

fn parse_input(input: &str) -> (Vec<u64>, Vec<Range<u64>>)
{
    let (ranges_raw, ingredients_raw): (&str, &str) = input.split_once("\n\n").unwrap();
    let ranges = ranges_raw.lines().map(|line| {
        let (start, stop) = line.split_once("-").unwrap();
        Range { start: start.parse::<u64>().unwrap(),
        end: stop.parse::<u64>().unwrap() + 1}
    }).collect::<Vec<Range<u64>>>();
    
    let ingredients = ingredients_raw.lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    // println!("n ranges: {}", ranges.len());
    // println!("n items: {}", ingredients.len());
    return (ingredients, ranges);
}

fn merge_overlapping_ranges(ranges_in: &Vec<Range<u64>>) ->Vec<Range<u64>>
{
    let mut ranges = ranges_in.clone();
    ranges.sort_by_key(|r| (r.start, r.end));
    let mut current = ranges[0].clone();
    let mut result: Vec<Range<u64>> = Vec::new();

    for interval in ranges.into_iter().skip(1){
        if interval.start < current.end {
            current.end = max(interval.end, current.end);
        } else {
           result.push(current);
           current = interval.clone();
        }
    }
    result.push(current);
    // println!("{:?}", result);
    return result;
}

fn count_fresh_items(ranges_in: &Vec<Range<u64>>) -> usize
{
    let ranges = merge_overlapping_ranges(ranges_in);
    println!("merged ranges: {}", ranges.len());
    let count = ranges.into_iter().fold(0, |acc, r| acc + (r.end - r.start) as usize);
    // println!("count: {}", count);
    return count;
}


#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn test_parse_input()
    {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        let (ingredients, ranges) = parse_input(input);
        assert!(ingredients == [1, 5, 8, 11, 17, 32]);
        assert!(ranges == [Range{start:3, end:6},
            Range{start:10, end:15},
            Range{start:16, end:21},
            Range{start:12, end:19}]);
    }

     #[test]
    fn test_count_all_fresh_items()
    {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        let (_, ranges) = parse_input(input);
        assert!(count_fresh_items(&ranges) == 14);
    }

}
