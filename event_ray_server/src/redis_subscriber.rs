use crate::error::Error;
use error_stack::{Report, ResultExt};
use event_ray_core::app_event::AppEvent;
use redis::Client;
use tokio::sync::broadcast;
use tokio_stream::StreamExt;

pub async fn run_redis_subscriber(
    redis_url: &str,
    channel: &str,
    event_sender: broadcast::Sender<AppEvent>,
) -> Result<(), Report<Error>> {
    // Connect to Redis
    let client = Client::open(redis_url)
        .map_err(|e| Report::new(Error::from(e)))
        .attach("Failed to create Redis client")?;

    let mut pubsub = client
        .get_async_pubsub()
        .await
        .map_err(|e| Report::new(Error::from(e)))
        .attach("Failed to get Redis pubsub connection")?;

    // Subscribe to the channel
    pubsub
        .subscribe(channel)
        .await
        .map_err(|e| Report::new(Error::from(e)))
        .attach(format!("Failed to subscribe to channel: {}", channel))?;

    eprintln!("Redis subscriber listening on channel: {}", channel);

    // Loop to receive messages
    let mut stream = pubsub.on_message();
    loop {
        let msg = stream
            .next()
            .await
            .ok_or_else(|| Report::new(Error::RedisStreamEnded))?;

        let payload: String = msg
            .get_payload()
            .map_err(|e| Report::new(Error::from(e)))?;

        // Deserialize the message
        match serde_json::from_str::<AppEvent>(&payload) {
            Ok(event) => {
                // Send to broadcast channel
                if let Err(e) = event_sender.send(event) {
                    eprintln!("No subscribers listening, dropping event: {:?}", e);
                }
            }
            Err(e) => {
                // Non-fatal: log and continue
                eprintln!("Failed to deserialize event from Redis: {:?}", e);
            }
        }
    }
}
