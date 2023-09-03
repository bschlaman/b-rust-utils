use chrono::{DateTime, Local, Utc};
use colored::Colorize;
use prettytable::{format, row, Attr, Cell, Row, Table};

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

fn print_datetime_table() {
    let utc = Utc::now();
    // don't call Local::now() so that a single time snapshot is used
    let local = utc.with_timezone(&Local);

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_BORDERS_ONLY);

    table.set_titles(Row::new(vec![
        Cell::new("Description")
            .with_style(Attr::Bold)
            .with_style(Attr::Italic(true)),
        Cell::new(""),
        Cell::new("Local")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(prettytable::color::RED)),
        Cell::new("UTC")
            .with_style(Attr::Bold)
            .with_style(Attr::ForegroundColor(prettytable::color::BLUE)),
        Cell::new("Length").with_style(Attr::Dim),
    ]));

    table.add_row(row![
        "Package name".italic(),
        "📦",
        env!("CARGO_PKG_NAME").bold(),
        "",
        "",
    ]);

    table.add_row(row![
        "Unix".italic(),
        "⏰️",
        utc.timestamp().to_string().yellow(),
        "",
        utc.timestamp().to_string().len().to_string().dimmed(),
    ]);

    table.add_empty_row();

    /*
    table.add_row(row![
        "Default".italic(),
        local.to_string_(),
        utc.to_string_(),
        utc.to_string_().len(),
    ]);
    */

    table.add_row(row![
        "Friendly Calendar".italic(),
        "📅",
        local.to_calendar(),
        utc.to_calendar().dimmed(),
        utc.to_calendar().len().to_string().dimmed(),
    ]);

    table.add_row(row![
        "RFC 3339".italic(),
        "⌛",
        local.to_rfc3339_secs(),
        utc.to_rfc3339_secs().blue().dimmed(),
        utc.to_rfc3339_secs().len().to_string().dimmed(),
    ]);

    table.add_row(row![
        "ISO 8601".italic(),
        "📄",
        local.to_iso8601(),
        utc.to_iso8601().dimmed(),
        utc.to_iso8601().len().to_string().dimmed(),
    ]);

    table.add_row(row![
        "ISO 8601 Date".italic(),
        "💻",
        local.to_iso8601_dots().yellow().bold(),
        utc.to_iso8601_dots().dimmed(),
        utc.to_iso8601_dots().len().to_string().dimmed(),
    ]);

    table.printstd();
}

fn main() {
    print_datetime_table();
}
