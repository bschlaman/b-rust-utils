use chrono::{Local, Utc};
use colored::Colorize;


fn print_dates_with_emojis() {
    let utc = Utc::now();
    let local = Local::now();
    
    let utc_iso_formatted = utc.to_rfc3339();
    let unix = local.timestamp();

    let utc_formatted = utc.format("%Y-%m-%d %H:%M");
    let utc_friendly = utc.format("%a %b %e %H:%M %Z");
    let local_formatted = local.format("%Y-%m-%d %H:%M");
    
    let package_name = env!("CARGO_PKG_NAME");

    println!("== {} ==", package_name.bold());
    println!("{}", unix.to_string().yellow());
    println!("{}", utc_iso_formatted.blue());
    println!("===========");
    println!("⌛ {} {}", local_formatted, "(Local)".dimmed());
    println!("⌛ {} {}", utc_formatted, "(UTC)".dimmed());
    println!("📅 {} {}", utc_friendly, "(UTC)".dimmed());
}

fn main() {
    print_dates_with_emojis();
}

