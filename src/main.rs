use actix_web::{App, HttpServer, web};
use actix_web::dev::Server;
use actix_web::web::Data;
use rust_rdkafka_demo::{collect_events, KafkaEventsBuffer};
use crate::lib::collect_events_durable;

pub mod lib;
#[actix_web::main]

// Use this block to switch between main blocking and non blocking modes.
/**
async fn initialize_nonblocking_server() {
    let kafka_producer = KafkaEventsBuffer::make_async_producer();
    HttpServer::new(move || App::new()
        .app_data(Data::new(kafka_producer.clone()))
        .route("/", web::post().to(collect_events)))
        .bind(("0.0.0.0", 8888))
        .expect("Unable to bind to the specified address.")
        .run().await
}

async fn initialize_blocking_server()  {
    let kafka_producer = KafkaEventsBuffer::make_producer();
    HttpServer::new(move || App::new()
        .app_data(Data::new(kafka_producer.clone()))
        .route("/", web::post().to(collect_events_durable)))
        .bind(("0.0.0.0", 8888))
        .expect("Unable to bind to the specified address.")
        .run().await
}
 */
async fn main() -> std::io::Result<()> {
    let kafka_producer = KafkaEventsBuffer::make_producer();
    HttpServer::new(move || App::new()
        .app_data(Data::new(kafka_producer.clone()))
        .route("/", web::post().to(collect_events_durable)))
        .bind(("0.0.0.0", 8888))
        .expect("Unable to bind to the specified address.")
        .run().await
}




