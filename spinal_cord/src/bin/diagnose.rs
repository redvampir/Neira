use std::time::Duration;

#[tokio::main]
async fn main() {
    let base = std::env::args().nth(1).unwrap_or("http://127.0.0.1:3000".into());
    let client = reqwest::Client::builder().timeout(Duration::from_secs(3)).build().unwrap();
    println!("Endpoint: {}", base);
    // status
    match client.get(format!("{}/api/neira/control/status", base)).send().await {
        Ok(r) => match r.text().await { Ok(t) => println!("status: {}", t), Err(_) => println!("status: <read err>") },
        Err(e) => println!("status error: {}", e),
    }
    // metrics
    match client.get(format!("{}/metrics", base)).send().await {
        Ok(r) => match r.text().await { Ok(_t) => println!("metrics: OK"), Err(_) => println!("metrics: <read err>") },
        Err(e) => println!("metrics error: {}", e),
    }
}

