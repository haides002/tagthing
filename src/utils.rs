use chrono::{DateTime, FixedOffset};


/// Parse date in either rfc3339 or rfc2822 format, return both parse errors as a tuple otherwise
///
/// First error in the tuple is from the rfc3339 standard, the second one is from the rfc2822
/// standard
pub fn parse_date(
    date: &str,
) -> Result<DateTime<FixedOffset>, (chrono::ParseError, chrono::ParseError)> {
    let rfc3339 = DateTime::parse_from_rfc3339(date);
    let rfc2822: Result<DateTime<FixedOffset>, chrono::ParseError>;
    if let Ok(parsed) = rfc3339 {
        return Ok(parsed);
    } else {
        rfc2822 = DateTime::parse_from_rfc2822(date);
        if let Ok(parsed) = rfc2822 {
            return Ok(parsed);
        }
    }
    Err((rfc3339.err().unwrap(), rfc2822.err().unwrap()))
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
