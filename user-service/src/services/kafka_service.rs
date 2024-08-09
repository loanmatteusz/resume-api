use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;

pub struct KafkaProducer {
  producer: FutureProducer,
}

impl KafkaProducer {
  pub fn new(brokers: &str) -> KafkaProducer {
    let producer: FutureProducer = ClientConfig::new()
      .set("bootstrap.servers", brokers)
      .create()
      .expect("Producer creation error");
    KafkaProducer { producer }
  }

  pub async fn send(&self, topic: &str, key: &str, payload: &str) {
    let record = FutureRecord::to(topic)
      .payload(payload)
      .key(key);

    let _ = self.producer.send(record, Duration::from_secs(0)).await;
  }
}
