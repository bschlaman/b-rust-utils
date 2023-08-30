use chrono::{Local, Utc, DateTime};
use colored::Colorize;
use prettytable::{row, Attr, Cell, Row, Table};

struct DateTimeFormatter {
    name: String,
    formatter: Box<dyn Fn(&DateTime<Utc>) -> String>,
}


fn also_ptable() {
    let _formatters: Vec<DateTimeFormatter> = vec![
        DateTimeFormatter {
            name: "Default".to_string(),
            formatter: Box::new(|dt: &DateTime<Utc>| dt.to_string()),
        },
        DateTimeFormatter {
            name: "RFC3339".to_string(),
            formatter: Box::new(|dt: &DateTime<Utc>| dt.to_rfc3339()),
        },
        DateTimeFormatter {
            name: "Custom".to_string(),
            formatter: Box::new(|dt: &DateTime<Utc>| dt.format("%Y-%m-%d %H:%M:%S").to_string()),
        },
    ];

    let utc = Utc::now();
    let local = utc.with_timezone(&Local);

    let mut table = Table::new();

    table.add_row(Row::new(vec![
        Cell::new("Description")
            .with_style(Attr::Bold),
        Cell::new("UTC")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(prettytable::color::YELLOW)),
        Cell::new("Local")
            .with_style(Attr::BackgroundColor(prettytable::color::RED))
            .with_style(Attr::Italic(true)),
        Cell::new("Length"),
    ]));

    table.add_row(row![
        "Package name",
        "",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_NAME").len()
    ]);

    table.add_row(row![
        "Default",
        "UTC",
        utc.to_string(),
        utc.to_string().len(),
    ]);

    table.add_row(row![
        "RFC 3339",
        "UTC",
        utc.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        utc.to_rfc3339_opts(chrono::SecondsFormat::Secs, true).len()
    ]);

    table.add_row(row![
        "ISO 8601",
        "UTC",
        utc.format("%Y.%m.%d").to_string(),
        utc.format("%Y.%m.%d").to_string().len()
    ]);

    table.add_row(row![
        "Local formatted",
        "Local",
        local.format("%Y-%m-%d %H:%M"),
        local.format("%Y-%m-%d %H:%M").to_string().len(),
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
