use std::fs;
use std::error::Error;
use std::ops::Range;
use intervaltree::IntervalTree;

fn main() ->  Result<(), Box<dyn Error>>
{
    println!("task 1");
    let input: String = fs::read_to_string("input.txt")?;

    let (ingredients, ranges) = parse_input(&input);
    let total = count_fresh(&ingredients, &ranges);

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

    println!("n ranges: {}", ranges.len());
    println!("n items: {}", ingredients.len());
    return (ingredients, ranges);
}

fn query_ranges(x: u64, ranges: &Vec<Range<u64>>) -> bool
{
    for range in ranges {
       if range.contains(&x) {
           return true;
       }
    }
    return false;
}

fn count_fresh(ingredients: &Vec<u64>, ranges: &Vec<Range<u64>>) -> usize
{
    // let db: IntervalTree<u64, u64> = ranges.iter().enumerate()
    //     .map(|(i, range)| {
    //         (range.clone(), i as u64)
    //     }).collect();
    //
    // ingredients.iter().filter(|x| !db.query_point(**x).next().is_some()).count()
    ingredients.iter().filter(|x| query_ranges(**x, ranges)).count()


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
    fn test_count()
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
        assert!(count_fresh(&ingredients, &ranges) == 3);
    }
}
