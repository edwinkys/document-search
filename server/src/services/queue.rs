use lapin::options::QueueDeclareOptions;
use lapin::types::FieldTable;
use lapin::{Channel, Connection, ConnectionProperties};

#[derive(Debug)]
pub struct Queue {
    name: String,
    channel: Channel,
}

impl Queue {
    /// Creates a new instance of the task queue.
    pub async fn new(name: impl AsRef<str>, url: impl AsRef<str>) -> Self {
        let url = url.as_ref();
        let property = ConnectionProperties::default();
        let connection = Connection::connect(url, property)
            .await
            .expect("Failed to connect to the queue");

        let channel = connection
            .create_channel()
            .await
            .expect("Failed to establish a channel");

        let name = name.as_ref();
        let options = QueueDeclareOptions::default();
        let table = FieldTable::default();
        channel
            .queue_declare(name, options, table)
            .await
            .expect("Failed to declare a queue");

        Queue {
            name: name.to_string(),
            channel,
        }
    }
}
