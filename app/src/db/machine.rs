type Ip = [u8; 4];

struct Machine {
    name: String,
    ip: Ip,
}

pub async fn add(machine: Machine) {
    todo!();
}

pub async fn update(name: String, new_ip: Ip) {
    todo!();
}

pub async fn delete(name: String) {
    todo!();
}


pub async fn get_ip(name: String) -> Ip {
    todo!();
}