use rumqttc::{AsyncClient, MqttOptions};
use std::time::Duration;
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let mut mqttoptions = MqttOptions::new("rumqtt-async", "test.mosquitto.org", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));

    let (client, _eventloop) = AsyncClient::new(mqttoptions, 10);
    let topic = format!("{}/publish_time_main", Uuid::new_v4());

    let _res = rumqttclib::publish_time(&client, &topic, 1, 5).await;
}
