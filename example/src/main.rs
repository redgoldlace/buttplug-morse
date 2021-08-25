use std::{sync::Arc, time::Duration};

// These are all imported as different names because they're ugly as sin otherwise. I already know they're
// buttplug-related. I don't need the name to remind me.
use buttplug::{
    client::ButtplugClient as Client,
    connector::{
        ButtplugRemoteClientConnector as RemoteClientConnector,
        ButtplugWebsocketClientTransport as WebsocketClientTransport,
    },
    core::messages::serializer::ButtplugClientJSONSerializer as ClientJSONSerializer,
};
use buttplug_morse::prelude::*;
use futures::future;

#[tokio::main]
async fn main() {
    // We need to create a connector before we do anything. Intiface will start a server on this address by default, so
    // it's what we use here.
    let connector = RemoteClientConnector::<WebsocketClientTransport, ClientJSONSerializer>::new(
        WebsocketClientTransport::new_insecure_connector("ws://127.0.0.1:12345"),
    );

    // Then we need to connect our client and start scanning for devices.
    let client = Client::new("morsecode");
    client.connect(connector).await.expect("failed to connect");
    client
        .start_scanning()
        .await
        .expect("scanning for devices failed");

    // It may take a little bit for devices to be discovered, so we'll sleep for 5 seconds.
    tokio::time::sleep(Duration::from_secs(5)).await;

    // This is an `Arc` so that it can be cheaply cloned between tasks.
    let options = Arc::new(MorseOptions::default());
    let mut futures = Vec::new();

    // Now that we're all set up, we can play some morse code on each device.
    for device in client.devices() {
        // Cloning an `Arc` is cheap.
        let options = Arc::clone(&options);

        // We need to keep track of each `JoinHandle`, so that we can join on them all later before we exit.
        futures.push(tokio::spawn(async move {
            // This is the bread and butter of `buttplug-morse`. Given a device, some options, and a string slice, it will
            // convert that string slice to morse code and play it on the provided device.
            play(&device, &options, "hello world!")
                .await
                .expect("playing message failed")
        }));
    }

    // We've started all of our tasks, but we need to join on them and make sure they complete before we exit.
    future::join_all(futures).await;
}
