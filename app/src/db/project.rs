use super::Uuid;

type Port = u16;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Project {
    pub name: String,
    pub port: Port,
    pub path: String,
    pub descr: String,
}