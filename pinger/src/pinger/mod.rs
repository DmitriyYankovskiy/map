use axum::http::request;
use tokio::{sync::Mutex, time};
use uuid::Uuid;

use std::{collections::HashMap, sync::Arc};

use crate::{db::models::Unit, State, Ip, Port};

#[derive(Clone)]
struct Tracker {
    unit_id: Uuid,
    ip: Ip, port: Port,
    last_ping: Arc<Mutex<time::Instant>>,
}

impl Tracker {
    pub fn new(unit_id: Uuid, ip: Ip, port: Port) -> Tracker {
        Tracker {
            unit_id,
            ip, port,
            last_ping: Arc::new(Mutex::new(time::Instant::now())),
        }
    }

    async fn track(&self, state: State, unit_id: Uuid) {
        loop {
            reqwest::get("")

            *self.last_ping.lock().await = time::Instant::now();

            time::sleep(time::Duration::from_secs(60)).await;
        }

        state.pinger.trackers.lock().await.remove(&unit_id);
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

    pub async fn track_unit(&self, state: State, unit_id: Uuid) -> Result<(), String> {
        let tracker = {
            let mut trackers = self.trackers.lock().await; 
            if trackers.contains_key(&unit_id) {
                return Err("this unit already track".to_string());
            }

            let unit = state.data.get_unit_by_id(unit_id)?;
            let port = unit.port;
            let ip = state.data.get_machine_by_id(Uuid::from_u128(unit.machine_id))?.ip;


            trackers.insert(unit_id, Tracker::new(unit_id, ip, port));
            trackers[&unit_id].clone()
        };

        tokio::spawn(async move {
            tracker.track(state.clone(), unit_id);
        });

        Ok(())
    }
}