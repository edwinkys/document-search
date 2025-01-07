use super::*;
use axum::http::StatusCode;
use lapin::options::{BasicPublishOptions, QueueDeclareOptions};
use lapin::types::FieldTable;
use lapin::{BasicProperties, Channel, Connection, ConnectionProperties};

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

    /// Publishes an extraction task to the queue.
    pub async fn publish(
        &self,
        task: &ExtractionTask,
    ) -> Result<(), ErrorResponse> {
        let name = self.name.as_str();
        let options = BasicPublishOptions::default();
        let properties = BasicProperties::default();

        // Unwrapping is safe because the task is serializable.
        let payload = serde_json::to_vec(task).unwrap();

        self.channel
            .basic_publish("", name, options, &payload, properties)
            .await
            .map_err(|_e| {
                #[cfg(test)]
                eprintln!("Failed to queue up the task: {_e:?}");
                ErrorResponse {
                    code: StatusCode::INTERNAL_SERVER_ERROR,
                    message: String::from("Failed to queue up the task."),
                    solution: None,
                }
            })?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lapin::options::QueuePurgeOptions;

    impl Queue {
        /// Removes all messages from the queue.
        pub async fn purge(&self) -> Result<(), ErrorResponse> {
            let name = self.name.as_str();
            let options = QueuePurgeOptions::default();

            self.channel
                .queue_purge(name, options)
                .await
                .map_err(|_e| {
                    #[cfg(test)]
                    eprintln!("Failed to purge the queue: {_e:?}");
                    ErrorResponse {
                        code: StatusCode::INTERNAL_SERVER_ERROR,
                        message: String::from("Failed to purge the queue."),
                        solution: None,
                    }
                })?;

            Ok(())
        }
    }
}
