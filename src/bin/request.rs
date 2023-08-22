use simple_logger::SimpleLogger;
use colored::Colorize;

struct ResponseData {
    http_status_code: u16,
    body_length: usize,
}

async fn perform_get_request(url: &String) -> Result<ResponseData, reqwest::Error> {
    let res = reqwest::get(url).await?;
    let http_status_code = res.status().as_u16();
    let body = res.text().await?;

    Ok(ResponseData {
        http_status_code,
        body_length: body.len(),
    })
}

#[tokio::main]
async fn main() {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Debug)
        .init()
        .unwrap();

    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        log::error!("too few args");
        std::process::exit(1);
    }

    let filename = &args[0];
    let url = &args[1];

    log::info!("script: {}", filename.bold());
    log::info!("url:    {}", url.bright_yellow());

    let start_time = std::time::Instant::now();
    let res_data = perform_get_request(url).await.unwrap();
    let duration = start_time.elapsed().as_millis();
    log::debug!("time elapsed (ms): {}", duration);

    log::info!(
        "http status code: {}, length of response body: {}",
        res_data.http_status_code.to_string().blue(),
        res_data.body_length.to_string().green(),
    );
}