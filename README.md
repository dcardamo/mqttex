# MQTT Example

This project has a single function called `publish_time` that will publish the
number of seconds since it was called and repeat that `count` times.

All of this in the file `lib.rs` and there is a test in there to connect to
`test.mosquitto.org` and publish to it.  It verifies that the publishes occur
by also subscribing with another connection and verifying that all the messages
come through.

You can look in the `Cargo.toml` file to see the dependencies.  Its using:

* tokio for the async executor
* paho mqtt for the mqtt client.  Might be worth looking at https://crates.io/crates/rumqttc instead
  for a pure rust mqtt client.   The paho mqtt version will build the C library automatically but does
  need some deps installed.  It "just worked" on my mac.  https://github.com/eclipse/paho.mqtt.rust.
  If the pure rust version were used instead then no external dependencies would be needed.

To build:  `cargo build`
To run tests: `cargo test`

If you don't have rust installed, you can get it from here:
https://rustup.rs/
