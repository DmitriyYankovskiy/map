use axum::http::request;
use tokio::{sync::Mutex, time};
use uuid::Uuid;

use std::{collections::HashMap, sync::Arc};

type Ip = [u8; 4];
type Port = u16;

#[derive(Clone)]
pub struct Tracker {
    ip: Ip, port: Port, url: String,
}

impl Tracker {
    pub fn new(unit_id: Uuid, ip: Ip, port: Port, url: String) -> Tracker {
        Tracker {
            ip, port, url,
        }
    }
}

pub struct Pinger {
    trackers: Mutex<HashMap<uuid::Uuid, Tracker>>,
}

impl Pinger {
    pub fn new() -> Pinger {
        Pinger {
            trackers: Mutex::new(HashMap::new()),
        }
    }

    pub async fn track_units(&self, new_trackers: Vec<(Uuid, Tracker)>) {
        let mut trackers = self.trackers.lock().await;
        for (id, tracker) in new_trackers {
            trackers.insert(id, tracker);
        }
    }

    pub async fn get_list(&self) -> Vec<Uuid> {
        let ids = {let keys: Vec<Uuid> = self.trackers.lock().await.keys().into_iter().map(|&key| key).collect(); keys};
        for id in &ids {
            let _ = self.check(id).await;
        }
        self.trackers.lock().await.keys().into_iter().map(|&key| key).collect()
    }

    pub async fn get_status(&self, id: &Uuid) -> bool {
        !self.check(id).await
    }



    async fn check(&self, id: &Uuid) -> bool {
        {
            let trackers = self.trackers.lock().await;
            let tracker = trackers.get(id);
            let tracker = if let Some(tracker) = tracker {
                tracker
            } else {
                return false;
            };

            let ip = tracker.ip;
            let ip = format!("{}.{}.{}.{}", ip[0], ip[1], ip[2], ip[3]);

            if let Ok(_) = reqwest::get(format!("http://{}:{}{}", ip, tracker.port, tracker.url )).await {
                return true;
            }
        }
        
        self.trackers.lock().await.remove(id);
        false
    }
}