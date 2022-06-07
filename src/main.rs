use actix_web::{App, HttpServer, web};
use actix_web::web::Data;
use rust_rdkafka_demo::{collect_events, KafkaEventsBuffer};
pub mod lib;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let kafka_producer = KafkaEventsBuffer::make_async_producer();
    HttpServer::new(move || App::new()
        .app_data(Data::new(kafka_producer.clone()))
        .route("/", web::post().to(collect_events)))
        .bind(("127.0.0.1", 8888)).expect("Unable to bind to the specified address.")
        .run()
        .await
}


