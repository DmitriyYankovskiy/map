type Ip = [u8; 4];

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Machine {
    pub name: String,
    pub ip: Ip,
}