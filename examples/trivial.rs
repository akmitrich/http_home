#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let resp = client
        .get("http://127.0.0.1:4083/")
        .send()
        .await
        .expect("Error: request failed");
    println!("{:#?}", resp);
    println!("Server says: '{}'", resp.text().await.unwrap());
}
