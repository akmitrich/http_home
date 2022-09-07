use http_home::smart_device::{DeviceDict, Device, Socket};

#[tokio::main]
async fn main() {
    let socket = Socket::new(220., 5., true);
    let client = reqwest::Client::new();
    println!("With correct request Server says: '{}'", show_update_request_to_server(&client, "R", "S", socket.into()).await);
    println!("But when request is incorrect Server says: '{}'", show_update_request_to_server(&client, "No room", "No device", Device::Unknown).await);
}

async fn show_update_request_to_server(client: &reqwest::Client, room_name: &str, device_name: &str, device: Device) -> String {
    let query: Vec<(String, String)> = device
        .device_dict()
        .into_iter()
        .collect();
    let resp = client
        .post(format!(
            "http://127.0.0.1:4083/update/{room_name}/{device_name}"
        ))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .query(&query)
        .send()
        .await
        .expect("Error: request failed");
    println!("Response: {:#?}\n", resp);

    resp.text().await.unwrap()
}