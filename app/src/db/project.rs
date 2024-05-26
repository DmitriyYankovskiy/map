type Port = u16;

struct Project {
    name: String,
    port: Port,
    path: String,
    descr: String,
}

pub async fn add(project: Project) {
    todo!();
}

pub async fn update_url(name: String, port: Port, path: String) {
    todo!();
}

pub async fn update_descr(name: String, descr: String) {
    todo!();
}

pub async fn delete(project: Project) {
    todo!();
}