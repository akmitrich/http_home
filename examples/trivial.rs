#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    let body = "room_name=R&device_name=S&data=socket%2F%2F%2Fon%2F%2F%2F20%2F%2F%2F5";
    let resp = client
        .post("http://127.0.0.1:4083/update")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Error: request failed");
    println!("{:#?}", resp);
}