use std::fs;
use std::error::Error;
use nalgebra::{DMatrix};
use std::ops::{Add, Mul};

fn main() ->  Result<(), Box<dyn Error>>
{
    println!("task 1");
    let input: String = fs::read_to_string("input.txt")?;

    let (m, ops) = parse_input(&input);
    let results = evaluate_all(m, ops);
    let total: u64 = results.into_iter().sum();

    println!("total: {}", total);
    return Ok(())
}

#[derive(Debug, PartialEq, Eq)]
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide
}

fn parse_input(input: &str) -> (DMatrix<u64>, Vec<Operation>)
{
    let mut lines = input.lines()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let operations = lines.pop().unwrap()
        .iter().map(|op| {
            match *op {
                "+" => Operation::Add,
                "-" => Operation::Subtract,
                "*" => Operation::Multiply,
                "/" => Operation::Divide,
                _ => panic!("Unexpected symbol in operations list")
            }
        }).collect::<Vec<Operation>>();

    let cols = lines[0].len();
    let rows = lines.len();

    let numbers = DMatrix::<u64>::from_row_iterator(rows, cols, 
        lines.into_iter().flat_map(|l| l.into_iter())
        .map(|x| x.parse::<u64>().unwrap())
    );

    return (numbers, operations);
}

fn evaluate_all(vals: DMatrix<u64>, ops: Vec<Operation>) -> Vec<u64>
{
    return ops.into_iter().enumerate().map(|(i, op)| {
        let col = vals.column(i);
        match op {
            Operation::Add => col.sum(),
            Operation::Multiply => col.product(),
            _ =>  panic!("unexpected operation")
        }
    }).collect::<Vec<u64>>();
}


#[cfg(test)]
mod tests 
{
    use super::*;

    #[test]
    fn test_parse_input()
    {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   + ";
        let (m, ops) = parse_input(input);
        let expected_vals = DMatrix::from_row_slice(3, 4,
            &[123, 328, 51, 64,
             45, 64, 387, 23,
             6, 98, 215, 314]);
        // println!("m = {:?}", m);
        // println!("expected = {:?}", expected_vals);
        assert!(m == expected_vals);

        assert!(m.column(0).as_slice() == &[123, 45, 6]);

        assert!(ops == [Operation::Multiply,
            Operation::Add,
            Operation::Multiply,
            Operation::Add]);
    }

    #[test]
    fn test_evaluate()
    {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   + ";
        let (m, ops) = parse_input(input);

        let results = evaluate_all(m, ops);
        assert!(results == [33210, 490, 4243455, 401]);
    }

}
