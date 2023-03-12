use chrono::NaiveDate;

/// Parses a string that represents a date. When a date
/// is unable to be determined, return `None`.
/// seems Tim wanted us to not use the chrono library to solve... whoops!
//fn flexible_date_parse(text: &str) -> Option<NaiveDate> {
//    let formats = [
//        "%Y-%m-%d", "%Y/%b/%d", "%d.%b.%Y", "%b.%d.%Y", "%d/%B/%Y", "%d/%m/%y", "%v", "%F", "%D",
//        "%x",
//    ];
//
//    for f in formats {
//        let dt = NaiveDate::parse_from_str(text, f);
//        if !dt.is_err() {
//            return Some(dt.unwrap());
//        }
//    }
//
//    None
//}

fn is_year(field: &str) -> bool {
    field.len() == 4 && field.chars().all(|x| x.is_ascii_digit())
}

fn flexible_date_parse(text: &str) -> Option<NaiveDate> {
    let text = text.trim();

    // to support other languages need to look at what I can do about is_ascii_alphanumeric
    if !text.bytes().any(|x| x.is_ascii_alphanumeric()) {
        return None;
    }

    let fields: Vec<_> = text.split(['/', '.', '-', ' '].as_slice()).collect();

    let mut year: Option<i32> = None;
    let mut month: Option<i32> = None;
    let mut day: Option<i32> = None;

    for field in fields.iter() {
        if field.len() < 3 {
            continue;
        }

        // handle additional locales here with '|'
        // aiming to support english, gaelige, german, spanish, italian
        // need to move from ascii checks to utf8 checks
        let m = match &field.to_lowercase()[..3] {
            "jan" | "éan" => 1,
            "feb" | "fea" => 2,
            "mar" | "már" => 3,
            "apr" | "aib" => 4,
            "may" | "bea" => 5,
            "jun" | "mei" => 6,
            "jul" | "iúi" => 7,
            "aug" | "lún" => 8,
            "sep" | "mea" => 9,
            "oct" | "dei" => 10,
            "nov" | "sam" => 11,
            "dec" | "nol" => 12,
            _ => continue,
        };
        month = Some(m);
    }

    for field in fields.iter() {
        if is_year(field) {
            year = field.parse::<i32>().ok();
            continue;
        }

        if month.is_some() {
            day = field.parse::<i32>().ok()
        } else {
            month = field.parse::<i32>().ok()
        }
    }

    match (year, month, day) {
        (Some(y), Some(m), Some(d)) => NaiveDate::from_ymd_opt(y, m as u32, d as u32),
        (Some(y), Some(m), None) => NaiveDate::from_ymd_opt(y, m as u32, 1),
        _ => None,
    }
}

fn main() {
    let dates = [
        "2010-12-11",
        "1999/Mar/02",
        "01.Mar.2021",
        "Mar.05.2021",
        "not a date",
        "23/03/23",
        "8-Jul-2001",
        "2001-07-08",
        "07/08/21",
        "Márta-01-1916",
        "02-Lúnasa-1844",
    ];

    for d in dates.iter() {
        println!("{} -> {:?}", d, flexible_date_parse(d));
    }
}

#[test]
fn ymd_hyphen() {
    assert_eq!(
        flexible_date_parse("2010-12-11"),
        NaiveDate::from_ymd_opt(2010, 12, 11)
    )
}

#[test]
fn ymd_slash() {
    assert_eq!(
        flexible_date_parse("1999/Mar/02"),
        NaiveDate::from_ymd_opt(1999, 3, 2)
    )
}

#[test]
fn dmy_dot() {
    assert_eq!(
        flexible_date_parse("01.Mar.2021"),
        NaiveDate::from_ymd_opt(2021, 3, 1)
    )
}

#[test]
fn mdy_dot() {
    assert_eq!(
        flexible_date_parse("Apr.05.2021"),
        NaiveDate::from_ymd_opt(2021, 4, 5)
    )
}

#[test]
fn invalid() {
    assert_eq!(flexible_date_parse("not a date"), None)
}
