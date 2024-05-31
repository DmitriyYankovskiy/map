use super::Uuid;

type Ip = [u8; 4];

pub struct Machine {
    pub name: String,
    pub ip: Ip,
}