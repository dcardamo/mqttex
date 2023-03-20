use std::time::Duration;
use tokio::time::sleep;

use paho_mqtt as mqtt;

/// Publish the time to the mqtt server every interval_seconds.  Returns the number of
/// published messages that were sent
///
/// # Arguments
///
/// * `client` the paho mqtt client
/// * `interval_seconds` the number of seconds between publishes
/// * `count` the number of publishes to do
pub async fn publish_time(
    client: &mqtt::AsyncClient,
    topic: &str,
    interval_seconds: u16,
    count: u16,
) -> Result<u16, mqtt::Error> {
    for sent_successfully in 0..count {
        let payload = format!("seconds since: {}", sent_successfully);
        let msg = mqtt::Message::new(topic, payload, mqtt::QOS_1);
        client.publish(msg).await?;
        // this sleep doesn't block any threads.  This async function will wait till
        // the number of seconds has passed, then it will allow the executor to continue
        // in this function
        sleep(Duration::from_secs(interval_seconds as u64)).await;
    }

    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::stream::StreamExt;
    use uuid::Uuid;

    async fn setup_client() -> mqtt::AsyncClient {
        let host = "mqtt://test.mosquitto.org:1883".to_string();

        // Create the client
        let client = mqtt::AsyncClient::new(host).expect("could not instantiate async client");

        // Connect with default options and wait for it to complete or fail
        // The default is an MQTT v3.x connection.
        client
            .connect(None)
            .await
            .expect("could not connect to server");

        client
    }

    #[tokio::test]
    /// This tests `publish_time`
    ///
    /// It does that by publishing the time `count` times and subscribing
    /// to read that same thing back to verify it happened.
    /// All of this happens against the publish test.mosquitto.org MQTT server.
    async fn test_publish_time() {
        // Create our publish and subscribe clients
        let publish_client = setup_client().await;
        let mut subscribe_client = setup_client().await;
        let mut subscribe_client_stream = subscribe_client.get_stream(25);
        let topic = format!("{}/test_publish_time", Uuid::new_v4());

        subscribe_client
            .subscribe(&topic, mqtt::QOS_1)
            .await
            .expect("could not subscribe");

        let count = 5;
        let res = publish_time(&publish_client, &topic, 1, count).await;
        assert!(res.is_ok());
        assert_eq!(res.unwrap(), count);

        // verify all the messages were sent
        for i in 0..count {
            let expected_payload = format!("seconds since: {}", i);
            let msg = subscribe_client_stream
                .next()
                .await
                .expect("should have got message");
            assert!(msg.is_some());
            assert_eq!(expected_payload, msg.unwrap().payload_str());
            println!("got msg {}", i);
        }

        // Disconnect from the broker
        publish_client
            .disconnect(None)
            .await
            .expect("could not disconnect publisher");
        subscribe_client
            .disconnect(None)
            .await
            .expect("could not disconnect subscriber");
    }
}
