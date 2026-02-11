use chrono::{Datelike, NaiveDate};
use std::collections::HashSet;

pub fn parse_date_any(input: &str) -> Option<NaiveDate> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }

    if let Some(date) = parse_iso_date(trimmed) {
        return Some(date);
    }

    if let Some(date) = parse_dmy_date(trimmed) {
        return Some(date);
    }

    None
}

pub fn build_date_variants(date: NaiveDate, max_variants: usize) -> Vec<String> {
    let mut out = HashSet::new();
    let day = date.day();
    let month = date.month();
    let year = date.year();
    let year2 = year % 100;

    let dd = format!("{:02}", day);
    let mm = format!("{:02}", month);
    let yy = format!("{:02}", year2);

    out.insert(format!("{year}-{mm}-{dd}"));
    out.insert(format!("{day}.{month}.{year}"));
    out.insert(format!("{dd}.{mm}.{year}"));
    out.insert(format!("{day}/{month}/{year}"));
    out.insert(format!("{dd}/{mm}/{year}"));
    out.insert(format!("{day}.{month}.{yy}"));
    out.insert(format!("{dd}.{mm}.{yy}"));
    out.insert(format!("{day}/{month}/{yy}"));
    out.insert(format!("{dd}/{mm}/{yy}"));

    let mut list: Vec<String> = out.into_iter().collect();
    list.sort();
    list.truncate(max_variants);
    list
}

pub fn build_date_variants_for_range(
    start: NaiveDate,
    end: NaiveDate,
    max_variants: usize,
    max_days: i64,
) -> Vec<String> {
    if end < start {
        return Vec::new();
    }
    let total_days = (end - start).num_days() + 1;
    if total_days > max_days {
        return Vec::new();
    }

    let mut out = HashSet::new();
    let mut current = start;
    for _ in 0..total_days {
        for variant in build_date_variants(current, max_variants) {
            out.insert(variant);
        }
        current = current
            .checked_add_signed(chrono::Duration::days(1))
            .unwrap_or(end);
    }

    let mut list: Vec<String> = out.into_iter().collect();
    list.sort();
    list
}

fn parse_iso_date(input: &str) -> Option<NaiveDate> {
    let normalized = input.replace('/', "-");
    NaiveDate::parse_from_str(&normalized, "%Y-%m-%d").ok()
}

fn parse_dmy_date(input: &str) -> Option<NaiveDate> {
    let normalized = input.replace('/', ".");
    if let Ok(date) = NaiveDate::parse_from_str(&normalized, "%d.%m.%Y") {
        return Some(date);
    }
    if let Ok(date) = NaiveDate::parse_from_str(&normalized, "%d.%m.%y") {
        let year = date.year();
        if year < 100 {
            let adjusted = NaiveDate::from_ymd_opt(year + 2000, date.month(), date.day())?;
            return Some(adjusted);
        }
        return Some(date);
    }
    None
}
