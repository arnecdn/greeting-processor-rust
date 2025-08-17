mod kafka_consumer;
mod settings;

use crate::kafka_consumer::ConsumeTopics;
use crate::settings::Settings;
use greeting_db_api::{greeting_command::GreetingCommandRepositoryImpl, init_db, migrate};
use opentelemetry::trace::TracerProvider;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_config = Settings::new();

    greeting_otel::init_otel(
        &app_config.otel_collector.oltp_endpoint,
        "greeting_processor",
        &app_config.kube.my_pod_name,
    )
    .await;

    let pool = Box::new(
        init_db(app_config.db.database_url.clone())
            .await
            .expect("Expected db pool"),
    );
    
    
    
    let repo = Box::new(
        GreetingCommandRepositoryImpl::new(pool.clone())
            .await
            .expect("failed"),
    );
    let mut consumer = kafka_consumer::KafkaConsumer::new(app_config, repo)
        .await
        .expect("Failed to create kafka consumer");

    consumer
        .consume_and_store()
        .await
        .expect("Error in kafka consumer");

    Ok(())
}
