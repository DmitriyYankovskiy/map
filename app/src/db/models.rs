use crate::{Ip, Port};

#[derive(serde::Serialize, serde::Deserialize)]
pub enum Status {
    Online,
    Ofline,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Unit {
    pub machine_id: u128,

    pub name: String,
    pub port: Port,
    pub path: String,
    pub descr: String,
    pub status: Status,
}


#[derive(serde::Serialize, serde::Deserialize)]
pub struct Machine {
    pub name: String,
    pub ip: Ip,
}