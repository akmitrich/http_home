use http_home::smart_device::{Device, DeviceDict, Thermometer};
use reqwest::Client;

#[tokio::main]
async fn main() {
    let client = Client::new();
    println!(
        "Add room 'new room': Server says '{}'",
        add_room(&client, "new room").await
    );
    println!(
        "Add device 'new thermo' in room 'new room': Server says '{}'",
        add_device(
            &client,
            "new room",
            "new thermo",
            Device::Thermometer(Thermometer::new(36.6))
        )
        .await
    );
    println!(
        "Add device in invalid room: Server says '{}'",
        add_device(&client, "no room", "new device", Device::Unknown).await
    );
    println!(
        "Remove device 'new thermo' from room 'new room': Server says '{}'",
        remove_device(&client, "new room", "new thermo").await
    );
    println!(
        "Remove room 'new room': Server says '{}'",
        remove_room(&client, "new room").await
    );
}

async fn add_room(client: &Client, room_name: &str) -> String {
    let resp = client
        .post(format!("http://127.0.0.1:4083/add_room/{room_name}"))
        .send()
        .await
        .expect("Error: request failed");
    println!("Response: {:#?}\n", resp);

    resp.text().await.unwrap()
}

async fn add_device(client: &Client, room_name: &str, device_name: &str, device: Device) -> String {
    let query: Vec<(String, String)> = device.device_dict().into_iter().collect();
    let resp = client
        .post(format!(
            "http://127.0.0.1:4083/add_device/{room_name}/{device_name}"
        ))
        .query(&query)
        .send()
        .await
        .expect("Error: request failed");
    println!("Response: {:#?}\n", resp);

    resp.text().await.unwrap()
}

async fn remove_device(client: &Client, room_name: &str, device_name: &str) -> String {
    let resp = client
        .post(format!(
            "http://127.0.0.1:4083/remove_device/{room_name}/{device_name}"
        ))
        .send()
        .await
        .expect("Error: request failed");
    println!("Response: {:#?}\n", resp);

    resp.text().await.unwrap()
}

async fn remove_room(client: &Client, room_name: &str) -> String {
    let resp = client
        .post(format!("http://127.0.0.1:4083/remove_room/{room_name}"))
        .send()
        .await
        .expect("Error: request failed");
    println!("Response: {:#?}\n", resp);

    resp.text().await.unwrap()
}
