use reqwest::Client;

#[tokio::main]
async fn main() {
    let client = Client::new();
    println!("Ask for room_list: Server says '{}'", ask_list(&client, "room_list").await);
    println!("Ask for device_list in room 'R': Server says '{}'", ask_list(&client, "device_list/R").await);
    println!("Ask for device_list in invalid room: Server says '{}'", ask_list(&client, "device_list/no room").await);
}

async fn ask_list(client: &Client, path: &str) -> String {
    let resp = client
        .get(format!("http://127.0.0.1:4083/{path}"))
        .send()
        .await
        .expect("Error: request failed");
    println!("{:#?}", resp);
    resp.text().await.unwrap()
}