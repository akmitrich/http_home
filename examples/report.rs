use reqwest::Client;

#[tokio::main]
async fn main() {
    let client = Client::new();
    let resp = client
        .get("http://127.0.0.1:4083/report")
        .send()
        .await
        .expect("Error: request failed");
    println!("{:#?}", resp);
    println!("Server reports:\n{}", resp.text().await.unwrap());
}
