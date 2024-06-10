mod kafka_consumer;
mod kafka_single_msg_consumer;

use std::io::Error;
use std::os::fd::AsRawFd;
use std::thread;
use std::time::Duration;
use chrono::{Local, NaiveDateTime};
use config::Config;
use derive_more::{Display, Error};
use dotenv::dotenv;
use env_logger::{Builder, Target};
use futures_util::StreamExt;
use log::{Level, LevelFilter, Metadata, Record};
use rdkafka::{ClientConfig, Message};
use rdkafka::consumer::{CommitMode, Consumer, StreamConsumer};
use serde::{Deserialize, Serialize};
use rand::{random, Rng};
use rdkafka::error::KafkaError;

#[tokio::main]
async fn main() {
    log::set_logger(&CONSOLE_LOGGER).expect("Not able to config logger");
    log::set_max_level(LevelFilter::Info);

    kafka_consumer::consume_and_print().await
}

static CONSOLE_LOGGER: ConsoleLogger = ConsoleLogger;

struct ConsoleLogger;

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{}: {} - {}", Local::now(),record.level(), record.args());
        }
    }

    fn flush(&self) {}
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GreetingMessage {
    id: String,
    to: String,
    from: String,
    heading: String,
    message: String,
    created: NaiveDateTime,
}

#[derive(Deserialize)]
pub(crate) struct Settings {
    pub(crate) kafka: Kafka,
}

impl Settings {
    pub fn new() -> Self {
        dotenv().ok();

        let settings = Config::builder()
            .add_source(config::File::with_name("./res/server").required(false))
            .add_source(config::Environment::with_prefix("APP").separator("__"))
            .build()
            .unwrap();

        settings.try_deserialize().unwrap()
    }
}

#[derive(Deserialize)]
pub(crate) struct Kafka {
    pub(crate) broker: String,
    pub(crate) topic: String,
    pub(crate) consumer_group: String,
    pub(crate) message_timeout_ms: String,
    // pub (crate) enable_idempotence: bool,
    // pub (crate) processing_guarantee: String,
    // pub (crate) number_of_consumers:i32
}