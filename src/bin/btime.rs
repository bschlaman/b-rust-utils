use chrono::{DateTime, Local, Utc};
use colored::Colorize;
use prettytable::{row, Attr, Cell, Row, Table};

trait DateTimeFormatter {
    fn to_string_(&self) -> String;
    fn to_rfc3339_secs(&self) -> String;
    fn to_iso8601(&self) -> String;
    fn to_iso8601_dots(&self) -> String;
    fn to_calendar(&self) -> String;
}

impl<Tz: chrono::TimeZone> DateTimeFormatter for DateTime<Tz>
where
    Tz::Offset: std::fmt::Display,
{
    fn to_string_(&self) -> String {
        self.to_string()
    }

    fn to_rfc3339_secs(&self) -> String {
        self.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
    }

    fn to_iso8601(&self) -> String {
        self.format("%Y-%m-%d %H:%M").to_string()
    }

    fn to_iso8601_dots(&self) -> String {
        self.format("%Y.%m.%d").to_string()
    }

    fn to_calendar(&self) -> String {
        self.format("%a %b %e %H:%M %Z").to_string()
    }
}

fn also_ptable() {
    let utc = Utc::now();
    // don't call Local::now() so that a single time snapshot is used
    let local = utc.with_timezone(&Local);

    let mut table = Table::new();

    table.add_row(Row::new(vec![
        Cell::new("Description").with_style(Attr::Bold),
        Cell::new("Local")
            .with_style(Attr::BackgroundColor(prettytable::color::RED))
            .with_style(Attr::Italic(true)),
        Cell::new("UTC")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(prettytable::color::YELLOW)),
        Cell::new("Length"),
    ]));

    table.add_row(row![
        "Package name",
        "",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_NAME").len(),
    ]);

    table.add_row(row![
        "Unix",
        "",
        utc.timestamp().to_string(),
        env!("CARGO_PKG_NAME").len(),
    ]);

    table.add_row(row![
        "Default",
        local.to_string_(),
        utc.to_string_(),
        utc.to_string_().len(),
    ]);

    table.add_row(row![
        "RFC 3339",
        local.to_rfc3339_secs(),
        utc.to_rfc3339_secs(),
        utc.to_rfc3339_secs().len(),
    ]);

    table.add_row(row![
        "ISO 8601",
        local.to_iso8601(),
        utc.to_iso8601(),
        utc.to_iso8601().len(),
    ]);

    table.add_row(row![
        "ISO 8601 Date",
        local.to_iso8601_dots(),
        utc.to_iso8601_dots(),
        utc.to_iso8601_dots().len(),
    ]);

    table.printstd();
}

fn print_dates_with_emojis() {
    let utc = Utc::now();
    let local = Local::now();

    let unix = utc.timestamp();
    let utc_iso_formatted = utc.to_rfc3339();

    let utc_formatted = utc.format("%Y-%m-%d %H:%M");
    let iso_date = utc.format("%Y.%m.%d");
    let utc_friendly = utc.format("%a %b %e %H:%M %Z");
    let local_formatted = local.format("%Y-%m-%d %H:%M");

    let cargo_pkg_name = env!("CARGO_PKG_NAME");

    println!("== {} ==", cargo_pkg_name.bold());
    println!("{}", unix.to_string().yellow());
    println!("{}", utc_iso_formatted.blue());
    println!("{}", iso_date.to_string().bold());
    println!("{}", "=".repeat(utc_iso_formatted.len()));
    println!("⌛ {local_formatted:20} {}", "(Local)".dimmed());
    println!("⌛ {utc_formatted:20} {}", "(UTC)".dimmed());
    println!("📅 {utc_friendly:20} {}", "(UTC)".dimmed());
}

fn main() {
    print_dates_with_emojis();
    also_ptable()
}
