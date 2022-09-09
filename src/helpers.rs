/// Checks if given string is a 4 digits number, like "1234" (not "-123", "123", or "12345")
fn is_valid_year(num: i16) -> bool {
    num.is_positive() && num < 10_000 && num > 999
}

/// Checks if two given string have the same first two letters, like "1320" and "1399"
fn century_equal(year1: i16, year2: i16) -> bool {
    let str1 = year1.to_string();
    let str2 = year2.to_string();
    str1[..2] == str2[..2]
}

/// Returns slice of the full year, like 85 from 1985
fn slice_year(year: i16) -> String {
    let slice = &year.to_string()[2..4];
    slice.to_string()
}

/// Formats the range of two years into the string, e.g. "1720–95", or "1720–1805", or "1720–"
/// Start year and dash are always present
/// It's supposed to be used for lifespans, meaning we always have birth, but may not have death
pub fn format_years_range_string(start_year: i16, finish_year: Option<i16>) -> String {
    match (start_year, finish_year) {
        (start, _) if !is_valid_year(start) => "".to_string(),
        (start, None) => format!("{}–", start),
        (start, Some(finish)) if !is_valid_year(finish) => format!("{}–", start),
        (start, Some(finish)) if century_equal(start, finish) => {
            format!("{}–{}", start, slice_year(finish))
        }
        (start, Some(finish)) => format!("{}–{}", start, finish),
    }
}

/// Formats the range of two years into a string, e.g. "1720–95", or "1720–1805", or "1720"
/// Both years can be present or absent, so it's a more generic, loose form
pub fn format_years_range_loose(start_year: Option<i16>, finish_year: Option<i16>) -> String {
    match (start_year, finish_year) {
        (Some(start), None) if is_valid_year(start) => format!("{}", start),
        (None, Some(finish)) => format!("{}", finish),
        (Some(start), Some(finish)) if is_valid_year(start) && !is_valid_year(finish) => {
            format!("{}", start)
        }
        (Some(start), Some(finish)) if !is_valid_year(start) && is_valid_year(finish) => {
            format!("{}", finish)
        }
        (Some(start), Some(finish)) if century_equal(start, finish) => {
            format!("{}–{}", start, slice_year(finish))
        }
        (Some(start), Some(finish)) => format!("{}–{}", start, finish),
        (_, _) => "".to_string(),
    }
}

/// Formats minutes into a string with hours and minutes, like "2h 35m"
pub fn format_work_length(length_in_minutes: Option<i16>) -> String {
    let length = length_in_minutes.unwrap_or(0);
    let hours = length / 60;
    let minutes = length % 60;

    match (hours, minutes) {
        (0, 0) => "".to_string(),
        (h, m) if h < 0 || m < 0 => "".to_string(),
        (0, m) => format!("{}m", m),
        (h, 0) => format!("{}h", h),
        (h, m) => format!("{}h {}m", h, m),
    }
}

#[cfg(test)]
mod tests {
    use crate::helpers::{
        century_equal, format_work_length, format_years_range_loose, format_years_range_string,
        is_valid_year,
    };

    #[test]
    fn is_valid_year_returns_true() {
        assert!(is_valid_year(1000));
        assert!(is_valid_year(1234));
        assert!(is_valid_year(9999));
    }

    #[test]
    fn is_valid_year_returns_false() {
        assert!(!is_valid_year(999));
        assert!(!is_valid_year(10000));
        assert!(!is_valid_year(0));
        assert!(!is_valid_year(-1));
    }

    #[test]
    fn century_equal_returns_true() {
        assert!(century_equal(1700, 1799));
        assert!(century_equal(1750, 1749));
    }

    #[test]
    fn century_equal_returns_false() {
        assert!(!century_equal(1699, 1700));
        assert!(!century_equal(1799, 1800));
        assert!(!century_equal(1200, 1500));
    }

    #[test]
    fn format_years_range_string_works() {
        assert_eq!(format_years_range_string(1900, Some(1902)), "1900–02");
        assert_eq!(format_years_range_string(1890, Some(1912)), "1890–1912");
        assert_eq!(format_years_range_string(1890, Some(1)), "1890–");
        assert_eq!(format_years_range_string(1, Some(1912)), "");
        assert_eq!(format_years_range_string(1990, None), "1990–");
        assert_eq!(format_years_range_string(-1, None), "");
    }

    #[test]
    fn format_years_range_loose_works() {
        assert_eq!(format_years_range_loose(Some(1900), Some(1902)), "1900–02");
        assert_eq!(
            format_years_range_loose(Some(1890), Some(1912)),
            "1890–1912"
        );
        assert_eq!(format_years_range_loose(Some(1890), Some(1)), "1890");
        assert_eq!(format_years_range_loose(Some(1), Some(1912)), "1912");
        assert_eq!(format_years_range_loose(Some(1990), None), "1990");
        assert_eq!(format_years_range_loose(Some(-1), None), "");
        assert_eq!(format_years_range_loose(None, Some(1900)), "1900");
        assert_eq!(format_years_range_loose(None, None), "");
    }

    #[test]
    fn format_work_length_works() {
        assert_eq!(format_work_length(Some(12)), "12m");
        assert_eq!(format_work_length(Some(59)), "59m");
        assert_eq!(format_work_length(Some(60)), "1h");
        assert_eq!(format_work_length(Some(62)), "1h 2m");
        assert_eq!(format_work_length(Some(123)), "2h 3m");
        assert_eq!(format_work_length(None), "");
    }
}
