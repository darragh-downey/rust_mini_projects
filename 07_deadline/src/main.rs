use chrono::prelude::*;

struct ImportantEvent {
    what: String,
    when: DateTime<Local>,
}

trait Deadline {
    fn is_passed(&self) -> bool;
}

impl Deadline for ImportantEvent {
    // TODO: implement trait
    fn is_passed(&self) -> bool {
        println!(
            "Checking if I missed {}... Supposedly it's on {}",
            self.what.to_string(),
            self.when.format("%d/%m/%Y")
        );

        let cur_local = Local::now();
        println!("Well today is {}...", cur_local.format("%d/%m/%Y"));

        self.when < cur_local
    }
}

fn main() {
    let missed_christmas = ImportantEvent {
        what: String::from("Christmas"),
        when: Local.with_ymd_and_hms(2020, 12, 25, 0, 0, 0).unwrap(),
    };

    if missed_christmas.is_passed() {
        println!("oh well, maybe next year");
    } else {
        println!("☃︎");
    }
}

#[test]
fn in_past() {
    use chrono::Duration;

    let event = ImportantEvent {
        what: String::from("friend's birthday"),
        when: Local::today() - Duration::hours(25),
    };

    assert!(event.is_passed())
}

#[test]
fn in_future() {
    use chrono::Duration;

    let event = ImportantEvent {
        what: String::from("friend's birthday"),
        when: Local::today() + Duration::hours(25),
    };

    assert!(!event.is_passed())
}
