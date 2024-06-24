use serde::Deserialize;
use tokio::sync::Mutex;
use uuid::Uuid;

use std::collections::HashMap;

type Ip = [u8; 4];
type Port = u16;

#[derive(Clone, Deserialize)]
pub struct Mark {
    ip: Ip, port: Port, url: String,
}

pub struct Pinger {
    marks: Mutex<HashMap<uuid::Uuid, Mark>>,
}

impl Pinger {
    pub fn new() -> Pinger {
        Pinger {
            marks: Mutex::new(HashMap::new()),
        }
    }

    pub async fn track(&self, new_marks: Vec<(Uuid, Mark)>) {
        let mut marks = self.marks.lock().await;
        for (id, mark) in new_marks {
            marks.insert(id, mark);
        }
    }

    pub async fn get_list(&self) -> Vec<Uuid> {
        let ids = {let keys: Vec<Uuid> = self.marks.lock().await.keys().into_iter().map(|&key| key).collect(); keys};
        for id in &ids {
            let _ = self.check(id).await;
        }
        self.marks.lock().await.keys().into_iter().map(|&key| key).collect()
    }

    pub async fn get_status(&self, id: &Uuid) -> bool {
        self.check(id).await
    }



    async fn check(&self, id: &Uuid) -> bool {
        let mut marks = self.marks.lock().await;
    
        let mark = if let Some(mark) = marks.get(id) {
            mark
        } else {
            return false;
        };

        let ip = mark.ip;
        let ip = format!("{}.{}.{}.{}", ip[0], ip[1], ip[2], ip[3]);

        if let Ok(_) = reqwest::get(format!("http://{}:{}{}", ip, mark.port, mark.url )).await {
            return true;
        }
        
        marks.remove(id);
        false
    }
}