use paho_mqtt as mqtt;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let host = "mqtt://test.mosquitto.org:1883".to_string();

    // Create the client
    let client = mqtt::AsyncClient::new(host).expect("could not instantiate async client");

    // Connect with default options and wait for it to complete or fail
    // The default is an MQTT v3.x connection.
    client
        .connect(None)
        .await
        .expect("could not connect to server");

    let topic = format!("{}/test_publish_time", Uuid::new_v4());

    let _res = paholib::publish_time(&client, &topic, 1, 5).await;
}
