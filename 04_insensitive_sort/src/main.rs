fn sort_usernames<T: AsRef<str> + std::cmp::Ord>(users: &mut Vec<T>) {
    users.sort_by_cached_key(|x| x.as_ref().to_lowercase());
}

fn main() {
    let mut users = vec!["Todd", "Amy", "mike99", "Jennifer", "alison"];

    println!("unsorted: {:?}", &users);

    sort_usernames(&mut users);
    println!("sorted: {:?}", &users);
}

#[test]
fn five_users() {
    let mut users = vec!["Todd", "Amy", "mike99", "Jennifer", "alison"];
    let expected = vec!["alison", "Amy", "Jennifer", "mike99", "Todd"];
    sort_usernames(&mut users);

    assert_eq!(users, expected);
}
