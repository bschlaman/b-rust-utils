mod mime;
use colored::Colorize;
use prettytable::{format, row, Attr, Cell, Row, Table};
use simple_logger::SimpleLogger;
use std::io::Read;

#[derive(Debug)]
struct ResponseData {
    http_status_code: u16,
    body_length: usize,
    headers: reqwest::header::HeaderMap,
}

impl ResponseData {
    fn to_summary_table(&self) {
        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_BORDERS_ONLY);

        table.set_titles(Row::new(vec![
            Cell::new("Response attribute")
                .with_style(Attr::Bold)
                .with_style(Attr::Italic(true)),
            Cell::new(""),
            Cell::new("Value")
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(prettytable::color::RED)),
        ]));

        table.add_row(row![
            "http status code".italic(),
            "🔢",
            self.http_status_code,
        ]);

        table.add_row(row!["Body size".italic(), "💾", self.body_length,]);

        table.add_empty_row();
        table.add_row(row!["HTTP Headers".bold(), "➖"]);

        table.add_row(row!["Num headers", "", self.headers.len()]);

        for (key, val) in self.headers.iter() {
            table.add_row(row![
                key.to_string().italic().dimmed(),
                "",
                val.to_str().unwrap(),
            ]);
        }

        table.printstd();
    }

    fn to_content_type_table(&self) {
        let content_type_header_val = self.headers.get("content-type").unwrap().to_str().unwrap();
        let content_type_data =
            mime::ContentType::from_header_value(content_type_header_val).unwrap();

        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

        table.set_titles(Row::new(vec![
            Cell::new("Directive")
                .with_style(Attr::Bold)
                .with_style(Attr::Italic(true)),
            Cell::new("Value")
                .with_style(Attr::Bold)
                .with_style(Attr::ForegroundColor(prettytable::color::RED)),
        ]));

        table.add_row(row![
            "type".italic(),
            content_type_data.media_type.type_.to_string().bold()
        ]);

        table.add_row(row![
            "subtype".italic(),
            content_type_data.media_type.subtype.bold()
        ]);

        for (key, val) in content_type_data.parameters {
            table.add_row(row![key, val.dimmed()]);
        }

        println!("content-type: {}", content_type_header_val.blue().italic());
        table.printstd();
    }
}

/*
async fn perform_get_request_async(url: &String) -> Result<ResponseData, reqwest::Error> {
    let res = reqwest::get(url).await?;
    let http_status_code = res.status().as_u16();
    let body = res.text().await?;

    Ok(ResponseData {
        http_status_code,
        body_length: body.len(),
    })
}
*/

/*
#[tokio::main]
async fn main_async() {
    let url = "";

    let start_time = std::time::Instant::now();

    let res_data = perform_get_request_async(&url.to_string()).await.unwrap();

    let duration = start_time.elapsed().as_millis();
    log::debug!("time elapsed (ms): {}", duration);

    log::info!(
        "http status code: {}, length of response body: {}",
        res_data.http_status_code.to_string().blue(),
        res_data.body_length.to_string().green(),
    );
}
*/

fn perform_get_request(url: &str) -> Result<ResponseData, Box<dyn std::error::Error>> {
    let mut res = reqwest::blocking::get(url)?;

    let headers = res.headers().clone();

    let mut body = Vec::new();
    let num_bytes = res.read_to_end(&mut body)?;

    Ok(ResponseData {
        http_status_code: res.status().as_u16(),
        body_length: num_bytes,
        headers,
    })
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup();

    let start_time = std::time::Instant::now();

    let url = std::env::args().nth(1).unwrap();
    let res = perform_get_request(&url)?;

    let duration = start_time.elapsed().as_millis();
    log::debug!("time elapsed (ms): {}", duration);

    res.to_summary_table();
    res.to_content_type_table();

    Ok(())
}

/// Create the logger and print some debug information
fn setup() {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .init()
        .unwrap();

    // yes, I parse args again elsewhere.  sue me
    let mut args = std::env::args();

    let path_of_executable = args.next().unwrap();
    let arg1 = args.next().expect("Must supply exactly one arg: <url>");

    log::debug!("path of executable: {}", path_of_executable.bold());
    log::debug!("url:                {}", arg1.bright_yellow());
}
