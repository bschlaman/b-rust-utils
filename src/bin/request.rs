use colored::Colorize;
use simple_logger::SimpleLogger;

struct ResponseData {
    http_status_code: u16,
    body_length: usize,
}

async fn perform_get_request_async(url: &String) -> Result<ResponseData, reqwest::Error> {
    let res = reqwest::get(url).await?;
    let http_status_code = res.status().as_u16();
    let body = res.text().await?;

    Ok(ResponseData {
        http_status_code,
        body_length: body.len(),
    })
}

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

fn main() {
    setup();
}

fn setup() {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .init()
        .unwrap();

    let args = std::env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        log::error!("too few args");
        std::process::exit(1);
    }

    let filename = &args[0];
    let url = &args[1];

    log::info!("script: {}", filename.bold());
    log::info!("url:    {}", url.bright_yellow());
}
