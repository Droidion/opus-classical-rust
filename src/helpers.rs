/// Checks if given string is a 4 digits number, like "1234" (not "-123", "123", or "12345")
fn valid_digits(str: String) -> bool {
    str.len() == 4 && str[0..1] != *"-"
}

/// Checks if two given string have the same first two letters, like "1320" and "1399"
fn century_equal(year1: i16, year2: i16) -> bool {
    let str1 = year1.to_string();
    let str2 = year2.to_string();
    str1[..2] == str2[..2]
}

fn slice_year(year: i16) -> String {
    let str = year.to_string();
    let slice = &str[2..4];
    slice.to_string()
}

/// Formats the range of two years into the string, e.g. "1720–95", or "1720–1805", or "1720–"
/// Start year and dash are always present
/// It's supposed to be used for lifespans, meaning we always have birth, but may not have death
pub fn format_years_range_string(start_year: i16, finish_year: Option<i16>) -> String {
    match (start_year, finish_year) {
        (start, _) if !valid_digits(start.to_string()) => "".to_string(),
        (start, None) => format!("{}–", start),
        (start, Some(finish)) if !valid_digits(finish.to_string()) => format!("{}–", start),
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
        (Some(start), None) if valid_digits(start.to_string()) => format!("{}", start),
        (None, Some(finish)) => format!("{}", finish),
        (Some(start), Some(finish))
            if valid_digits(start.to_string()) && !valid_digits(finish.to_string()) =>
        {
            format!("{}", start)
        }
        (Some(start), Some(finish))
            if !valid_digits(start.to_string()) && valid_digits(finish.to_string()) =>
        {
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
