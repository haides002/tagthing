use chrono::{DateTime, FixedOffset};

/// Parse date in either rfc3339 or rfc2822 format, return both parse errors as a tuple otherwise
///
/// First error in the tuple is from the rfc3339 standard, the second one is from the rfc2822
/// standard
pub fn parse_date(date: &str) -> Option<DateTime<FixedOffset>> {
    let other_formats = [
        "%+",
        "%Y-%m-%dT%H:%M:%S",
        "%Y:%m:%d %H:%M:%S",
        "%Y:%m:%d %H:%M:%S%.f%:z",
    ];
    if let chrono::ParseResult::Ok(parsed) = DateTime::parse_from_rfc3339(date) {
        Some(parsed)
    } else if let chrono::ParseResult::Ok(parsed) = DateTime::parse_from_rfc2822(date) {
        dbg!(&parsed);
        Some(parsed)
    } else if let Some(parsed) = other_formats
        .map(|format| DateTime::parse_from_str(date, format))
        .iter()
        .find(|parsed| parsed.is_ok())
    {
        dbg!(&parsed);
        Some(parsed.unwrap())
    } else {
        dbg!(date);
        None
    }
}

macro_rules! benchmark {
    ($func:expr, $num:expr) => {
        let now = std::time::Instant::now();
        for _ in 0..$num {
            let _ = $func;
        }
        println!("{}", now.elapsed().as_millis());
    };
}
