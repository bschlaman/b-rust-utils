use env_logger::Env;
use log::{info, warn};
use std::env;
use std::io::Read;
use std::mem;

struct ResponseData {
    http_status_code: u16,
    body_size_bytes: usize,
}

fn perform_get_request(url: &String) -> Result<ResponseData, reqwest::Error> {
    let mut res = reqwest::blocking::get(url)?;
    let mut body = String::new();
    res.read_to_string(&mut body).ok();
    return Ok(ResponseData {
        http_status_code: res.status().as_u16(),
        body_size_bytes: mem::size_of_val(&mut body),
    });
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    info!("starting script!");
    warn!("I suck at rust!");

    let args: Vec<String> = env::args().collect();
    let filename = &args[0];
    let url = &args[1];

    info!("script name: {}", filename);
    info!("url passed: {}", url);

    let res_data = perform_get_request(url).unwrap();
    info!(
        "{}, {}",
        res_data.http_status_code, res_data.body_size_bytes
    );
}
