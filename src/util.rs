use crate::error::AlipayResult;

fn get_hour_min_sec(timestamp: u64) -> (i32, i32, i32) {
    let hour = ((timestamp % (24 * 3600)) / 3600 + 8) % 24;
    let min = (timestamp % 3600) / 60;
    let sec = (timestamp % 3600) % 60;
    (hour as i32, min as i32, sec as i32)
}

fn get_moth_day(is_leap_year: bool, mut days: i32) -> (i32, i32) {
    let p_moth: [i32; 12] = if is_leap_year {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };
    let mut day = 0;
    let mut moth = 0;

    for (i, v) in p_moth.iter().enumerate() {
        let temp = days - v;
        if temp <= 0 {
            moth = i + 1;
            day = if temp == 0 { *v } else { days };
            break;
        }
        days = temp;
    }
    (moth as i32, day)
}

pub fn datetime() -> AlipayResult<String> {
    use std::time::SystemTime;
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_secs();
    let days = 24 * 3600;
    let four_years = 365 * 3 + 366;
    let days = timestamp / days + (if (timestamp % days) != 0 { 1 } else { 0 });
    let year_4 = days / four_years;
    let mut remain = days % four_years;
    let mut year = 1970 + year_4 * 4;

    let mut is_leap_year = false;

    if (365..365 * 2).contains(&remain) {
        year += 1;
        remain -= 365;
    } else if (365 * 2..365 * 3).contains(&remain) {
        year += 2;
        remain -= 365 * 2;
    } else if 365 * 3 <= remain {
        year += 3;
        remain -= 365 * 3;
        is_leap_year = true;
    }

    let (moth, day) = get_moth_day(is_leap_year, remain as i32);
    let (h, m, s) = get_hour_min_sec(timestamp);
    Ok(format!(
        "{}-{:>02}-{:>02} {:>02}:{:>02}:{:>02}",
        year, moth, day, h, m, s,
    ))
}
