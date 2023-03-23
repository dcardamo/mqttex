use std::time::Duration;
use tokio::time::sleep;

use rumqttc::{AsyncClient, QoS};

/// Publish the time to the mqtt server every interval_seconds.  Returns the number of
/// published messages that were sent
///
/// # Arguments
///
/// * `client` the paho mqtt client
/// * `interval_seconds` the number of seconds between publishes
/// * `count` the number of publishes to do
pub async fn publish_time(
    client: &AsyncClient,
    topic: &str,
    interval_seconds: u16,
    count: u16,
) -> u16 {
    for sent_successfully in 0..count {
        let payload = format!("seconds since: {}", sent_successfully);

        client
            .publish(topic, QoS::AtLeastOnce, false, payload.clone())
            .await
            .unwrap();
        println!("published {} to {}", payload, topic);

        // this sleep doesn't block any threads.  This async function will wait till
        // the number of seconds has passed, then it will allow the executor to continue
        // in this function
        sleep(Duration::from_secs(interval_seconds as u64)).await;
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;
    use rumqttc::{AsyncClient, EventLoop, MqttOptions, QoS};
    use uuid::Uuid;

    async fn setup_client() -> (AsyncClient, EventLoop) {
        let mut mqttoptions = MqttOptions::new("rumqtt-async", "test.mosquitto.org", 1883);
        mqttoptions.set_keep_alive(Duration::from_secs(5));

        let (client, eventloop) = AsyncClient::new(mqttoptions, 10);

        (client, eventloop)
    }

    #[tokio::test]
    /// This tests `publish_time`
    ///
    /// It does that by publishing the time `count` times and subscribing
    /// to read that same thing back to verify it happened.
    /// All of this happens against the publish test.mosquitto.org MQTT server.
    async fn test_publish_time() {
        // Create our publish and subscribe clients
        let (publish_client, mut _pub_eventloop) = setup_client().await;
        let (subscribe_client, mut sub_eventloop) = setup_client().await;
        let topic = format!("{}/test_publish_time", Uuid::new_v4());

        subscribe_client
            .subscribe(&topic, QoS::AtMostOnce)
            .await
            .unwrap();

        let count = 5;
        let res = publish_time(&publish_client, &topic, 1, count).await;
        assert_eq!(res, count);

        // verify all the messages were sent
        for i in 0..count {
            let _expected_payload = format!("seconds since: {}", i);
            let msg = sub_eventloop.poll().await.unwrap();
            println!("got msg {:?}", msg);
        }
    }
}
