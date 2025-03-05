use nostr_sdk::client::Error as NostrClientError;
use nostr_sdk::event::Error as NostrEventError;
use thiserror::Error;
use nostr_sdk::prelude::*;
use std::vec;
use tokio::time::{sleep, Duration};

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum NostrError {
    #[error("Failed to fetch event: {0}")]
    EventFetchError(#[from] NostrEventError),

    #[error("Invalid Nostr key format")]
    InvalidKeyFormat,

    #[error("failed to add relay: {0}")]
    FailedToAddRelay(#[from] NostrClientError),

    #[error("Relay connection failed")]
    RelayConnectionFailed,
}

#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum UiError {
    #[error("Failed to render component: {0}")]
    RenderError(String),

    #[error("Invalid user input: {0}")]
    InvalidInput(String),
}

//npub1tp874205p5grs8ypscuw5x66anng62mqvdskyzwsdjjfddn9h6dsrhw8wt
//nsec1cjrj879z63wwxw3mva64pgjv538447k3kmaes0df3v89a5g0ypkq3d85em
//c48723f8a2d45ce33a3b677550a24ca44f5afad1b6fb983da98b0e5ed10f206c (priv key hex)
//584feaa9f40d10381c818638ea1b5aece68d2b6063616209d06ca496b665be9b (pub key hex)
const PRIVATE_KEY: &str = "nsec1cjrj879z63wwxw3mva64pgjv538447k3kmaes0df3v89a5g0ypkq3d85em";
const RELAY: &str = "wss://relay.damus.io";

// pub fn generate() -> Result<()> {
//     let keys = Keys::generate();
//
//     let public_key = keys.public_key();
//     let secret_key = keys.secret_key();
//
//     println!("Public key (hex): {}", public_key);
//     println!("Secret key (hex): {}", secret_key.to_secret_hex());
//
//     println!("Public key (bech32): {}", public_key.to_bech32()?);
//     println!("Secret key (bech32): {}", secret_key.to_bech32()?);
//
//     Ok(())
// }


async fn nostr_stuff() -> Result<()> {
    //TODO create a flow to check if user is new to NOSTR
    //TODO and redirect to route
    let secret_key = SecretKey::parse(PRIVATE_KEY).unwrap();
    let key_pair = Keys::new(secret_key);
    // let pub_key = key_pair.public_key();

    // connect to the client
    let client = Client::new(key_pair.clone());
    if let Err(e) = client.add_relay(RELAY).await {
        println!("{:?}", NostrError::FailedToAddRelay(e))
    };
    client.connect().await;
    let note_id = "note1f8ravzhkdht4vgyke6nnu24f43mrq2sjgwxcqy0eex4q47x55e7qtsucyy";
    let note_event_id = EventId::from_bech32(note_id)?;

    // Now create and publish a text note
    // let builder = EventBuilder::text_note("Just registered on tunani");
    // let event_builder_result = client.send_event_builder(builder).await.unwrap();

    //sleep so relay can buffere before fetching note
    sleep(Duration::from_secs(3)).await;

    let deletion_event = EventBuilder::delete(vec![note_event_id])
        .sign_with_keys(&key_pair)
        .expect("could not delete note");

    // fetch note using event id
    // let note_id = event_builder_result.id().to_bech32().unwrap();
    let deletion_event_id = deletion_event.id;
    let list_of_ids = vec![note_event_id, deletion_event_id];

    // create filter for event id
    let filter = Filter::new().ids(list_of_ids);

    //subscribe to and fetch note
    let events_from_relay = client
        .fetch_events_from(vec![RELAY], filter, Duration::from_secs(2))
        .await
        .unwrap();

    //listen to and fetch event
    for event in events_from_relay {
        println!("{:#?}", event);
    }

    Ok(())
}

