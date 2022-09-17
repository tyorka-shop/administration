use chrono::Utc;
use tokio::task::JoinHandle;
use tokio_schedule::{every, Job};

use crate::feed;

pub fn init() -> JoinHandle<()>{
    let every_day = every(1).days().at(12, 00, 00).in_timezone(&Utc).perform(|| async {
        let cfg = config::load("tyorka-admin");
        feed::sync(&cfg).await.unwrap();
    });

    tokio::spawn(every_day)
}
