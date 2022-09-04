#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let query = [("device", "socket"), ("state", "on")];
    let resp = client
        .post("http://127.0.0.1:4083/update/А/Б")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .query(&query)
        .send()
        .await
        .expect("Error: request failed");
    println!("{:#?}", resp);
}
