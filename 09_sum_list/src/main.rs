fn sum_with_missing_my_version(numbers: Vec<Option<i32>>) -> i32 {
    // always return a number i.e. 0 base case for series of None
    let mut sum = 0;

    for i in numbers {
        if i.is_some() {
            sum += i.unwrap();
        }
    }
    sum
}

// Tim's version
fn sum_with_missing(numbers: Vec<Option<i32>>) -> i32 {
    numbers.iter().map(|x| x.unwrap_or(0)).sum()
}

fn main() {
    let nn = vec![Some(1), Some(5), Some(4)];
    let s = sum_with_missing(nn);
    println!("{}", s);
}

#[test]
fn empty() {
    let nn = vec![];
    assert_eq!(sum_with_missing(nn), 0);
}

#[test]
fn no_missing() {
    let nn = vec![Some(1), Some(5), Some(4)];
    assert_eq!(sum_with_missing(nn), 10);
}

#[test]
fn some_missing() {
    let nn = vec![None, Some(1), Some(5), Some(4), None, None];
    assert_eq!(sum_with_missing(nn), 10);
}

#[test]
fn all_missing() {
    let nn = vec![None, None, None];
    assert_eq!(sum_with_missing(nn), 0);
}
