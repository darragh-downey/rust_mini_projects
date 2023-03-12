use std::vec;

// basic
//fn uniq_0(mut list: Vec<i32>) -> Vec<i32> {
//    list.sort();
//    list.dedup();
//    list
//}

// use generics implements Ord
// retain elements in their original order

// advanced 1: use generic types
fn uniq_1<T: Ord>(mut list: Vec<T>) -> Vec<T> {
    list.sort();
    list.dedup();

    list
}

// advanced 2: keep items in original order
fn uniq_2<T: Ord>(mut list: Vec<T>) -> Vec<T> {
    todo!()
}

// advanced 3: use iterators
fn uniq_3<T: Ord>(mut list: Vec<T>) -> Vec<T> {
    todo!()
}

fn main() {
    let input = vec![2, 1, 1];
    let answer = uniq_1(input);
    println!("uniq items -> {:?}", answer);
}

#[test]
fn empty_list() {
    let input: Vec<i32> = vec![];
    let expected_output: Vec<i32> = vec![];
    let actual_output: Vec<i32> = uniq_1(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1, 4, 5];
    let expected_output = vec![1, 4, 5];
    let actual_output = uniq_1(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1, 5, 2];
    let expected_output = vec![1, 2, 5];
    let actual_output = uniq_1(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list_with_duplicates() {
    let input = vec![1, 5, 2, 2, 1];
    let expected_output = vec![1, 2, 5];
    let actual_output = uniq_1(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list_with_duplicates() {
    let mut input = vec![1, 5, 2, 2, 1];
    input.sort_by(|x, y| x.partial_cmp(y).unwrap());
    let expected_output = vec![1, 2, 5];
    let actual_output = uniq_1(input);
    assert_eq!(actual_output, expected_output);
}
