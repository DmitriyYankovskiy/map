use super::Uuid;

type Ip = [u8; 4];

struct Machine {
    name: String,
    ip: Ip,
}

pub async fn add(machine: Machine) {
    todo!();
}

pub async fn update(id: Uuid, upd_machine: Machine) {
    todo!();
}

pub async fn delete(id: Uuid) {
    todo!();
}


pub async fn get(id: Uuid) -> Machine {
    todo!();
}