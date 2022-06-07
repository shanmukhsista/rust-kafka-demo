use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread::Thread;
use std::time::Duration;
use actix_web::{web, Result};
use actix_web::web::{Data};
use rdkafka::ClientConfig;
use rdkafka::producer::{BaseRecord, DefaultProducerContext, FutureProducer, FutureRecord, ThreadedProducer};
use serde_json::Value;
use serde::{Deserialize, Serialize};
use uuid::Uuid;


pub struct ProducerConfig {
    ack_timeout: u64,
    brokers: Vec<String>,
}

pub struct KafkaEventsBuffer {}

impl KafkaEventsBuffer {
    pub fn make_async_producer() -> Arc<Mutex<ThreadedProducer<DefaultProducerContext>>> {
        let producer_config = ProducerConfig { ack_timeout: 1, brokers: vec!["localhost:9092".to_string()] };
        let mut kafka_config = ClientConfig::new();
        let producer = kafka_config
            .set("message.timeout.ms", "2000")
            .set("bootstrap.servers", producer_config.brokers.join(",").to_string())
            .create::<ThreadedProducer<DefaultProducerContext>>().expect("An error occured while initializing producer.");
        Arc::new(Mutex::new(producer))
    }

    pub fn make_producer() -> FutureProducer {
        let producer_config = ProducerConfig { ack_timeout: 1, brokers: vec!["localhost:9092".to_string()] };
        let mut kafka_config = ClientConfig::new();
        let producer = kafka_config
            .set("message.timeout.ms", "2000")
            .set("bootstrap.servers", producer_config.brokers.join(",").to_string())
            .create::<FutureProducer>().expect("An error occured while initializing producer.");

        producer
    }
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Event {
    pub client_id: String,
    pub name: String,
    pub data: HashMap<String, Value>,
}


pub async fn collect_events(req_body: web::Json<Event>, kafka_producer: Data<Arc<Mutex<ThreadedProducer<DefaultProducerContext>>>>) -> Result<String> {
    let event = req_body.0;
    let event_id = Uuid::new_v4().to_string();
    let local_producer = kafka_producer.get_ref().lock().unwrap();
    local_producer.send(
        BaseRecord::to("events.main")
            .key(&event_id)
            .payload(&serde_json::to_string(&event).unwrap()));
    Ok(format!("{}", event_id))
}