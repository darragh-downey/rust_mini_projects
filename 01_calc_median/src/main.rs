use std::vec;

fn median(mut list: Vec<f32>) -> Option<f32> {
    // empty
    // odd number of elements
    // even number of elements

    if list.is_empty() {
        return None;
    }

    list.sort_by(|x, y| x.partial_cmp(y).unwrap());

    let n_elements = list.len();
    let middle = n_elements / 2;

    let med = if n_elements % 2 == 0 {
        // even number of elements
        (list[middle - 1] + list[middle]) / 2.0
    } else {
        list[middle]
    };

    Some(med)
}

fn main() {
    println!("Hello, world!");

    let answer = median(vec![1.0, 2.0, 5.0]);

    println!("median([1,2,5]) = {:?}", answer);
}

#[test]
fn empty_list() {
    let input = vec![];
    let expected_output = None;
    let actual_output = median(input);

    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1.0, 4.0, 5.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);

    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![5.0, 1.0, 4.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);

    assert_eq!(actual_output, expected_output);
}

#[test]
fn even_length() {
    let input = vec![5.0, 1.0, 4.0, 0.0];
    let expected_output = Some(2.5);
    let actual_output = median(input);

    assert_eq!(actual_output, expected_output);
}
