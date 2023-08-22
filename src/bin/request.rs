use simple_logger::SimpleLogger;
use std::env;
use std::io::Read;
use std::process::exit;

struct ResponseData {
    http_status_code: u16,
    body_length: usize,
}

fn perform_get_request(url: &String) -> Result<ResponseData, reqwest::Error> {
    let mut res = reqwest::blocking::get(url)?;
    let mut body = String::new();
    res.read_to_string(&mut body).ok();

    Ok(ResponseData {
        http_status_code: res.status().as_u16(),
        body_length: body.len(),
    })
}

#[tokio::main]
async fn main() {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .init()
        .unwrap();

    log::info!("starting script!");
    log::warn!("I suck at rust!");

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        log::error!("too few args");
        exit(1);
    }

    let filename = &args[0];
    let url = &args[1];

    log::info!("script name: {}", filename);
    log::info!("url passed:  {}", url);

    let res_data = perform_get_request(url).unwrap();
    log::info!(
        "http status code: {}, length of response body: {}",
        res_data.http_status_code,
        res_data.body_length,
    );
}
