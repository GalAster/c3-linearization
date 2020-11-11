use c3_linearization::{merge, C3};
use std::collections::HashMap;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    println!("{:#?}", merge(vec![vec!["B"], vec!["C", "D"]]))
}

#[test]
fn basic() {
    let c3 = C3::default();
    let mut input = HashMap::new();
    input.insert('A', vec!['B', 'C']);
    input.insert('B', vec![]);
    input.insert('C', vec!['D']);
    input.insert('D', vec![]);
    let mut target = HashMap::new();
    target.insert('A', vec!['A', 'B', 'C', 'D']);
    target.insert('B', vec!['B']);
    target.insert('C', vec!['C', 'D']);
    target.insert('D', vec!['D']);
    assert_eq!(target, c3.linearize(input).unwrap())
}
