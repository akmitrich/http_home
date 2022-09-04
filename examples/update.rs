use http_home::smart_device::DeviceDict;

#[tokio::main]
async fn main() {
    let socket = http_home::smart_device::Socket::new(220., 5., true);
    let client = reqwest::Client::new();
    let query: Vec<(String, String)> = http_home::smart_device::Device::Socket(socket)
        .device_dict()
        .into_iter()
        .collect();
    let room_name = "R";
    let device_name = "S";
    let resp = client
        .post(format!(
            "http://127.0.0.1:4083/update/{room_name}/{device_name}"
        ))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .query(&query)
        .send()
        .await
        .expect("Error: request failed");
    println!("{:#?}", resp);
}
