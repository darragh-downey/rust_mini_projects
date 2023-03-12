use chrono::NaiveDate;

fn weeks_between(a: &str, b: &str) -> i32 {
    // error case to return -1
    let d1 = NaiveDate::parse_from_str(a, "%Y-%m-%d").expect("Invalid date");
    let d2 = NaiveDate::parse_from_str(b, "%Y-%m-%d").expect("Invalid date");

    let duration = d2 - d1;
    let weeks = duration.num_weeks();

    weeks.try_into().unwrap_or(-1)
}

fn main() {
    let n_weeks = weeks_between("2010-01-21", "2010-10-21");

    println!("hello: {}", n_weeks);
}

#[test]
fn same_day() {
    let n_weeks = weeks_between("1010-10-10", "1010-10-10");
    assert_eq!(n_weeks, 0);
}

#[test]
fn one_week() {
    let n_weeks = weeks_between("1010-10-10", "1010-10-18");
    assert_eq!(n_weeks, 1);
}

#[test]
fn past() {
    let n_weeks = weeks_between("1010-10-18", "1010-10-10");
    assert_eq!(n_weeks, -1);
}
