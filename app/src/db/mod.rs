pub mod machine;
pub mod project;

use uuid::Uuid;
use project::Project;
use machine::Machine;

pub trait Data {
    fn get_project_by_id(&self, id: Uuid) -> Project;
    fn create_project(&self, new_project: &Project);
    fn delete_project(&self, id: Uuid);
    fn update_project(&self, id: Uuid, upd_project: &Project);

    fn get_machine_by_id(&self, id: Uuid) -> Machine;
    fn create_machine(&self, new_machine: &Machine);
    fn delete_machine(&self, id: Uuid);
    fn update_machine(&self, id: Uuid, new_machine: &Machine);
}


#[derive(Clone)]
pub struct Postgres {
}


impl Data for Postgres {
    fn get_project_by_id(&self, id: Uuid) -> Project {
        Project {
            name: "a".to_string(),
            port: 6666,
            path: "/pppppp".to_string(),
            descr: "description".to_string(),
        }
    }

    fn create_project(&self, new_project: &Project) {
        println!(">>> create project");
    }


    fn delete_project(&self, id: Uuid) {
        println!(">>> delete {{{}}} project", id.as_u128());
    }

    fn update_project(&self, id: Uuid, upd_project: Project) {
        println!(">>> update {{{}}} project", id.as_u128());
    }


    fn get_machine_by_id(&self, id: Uuid) -> Machine {
        Machine {
            name: "a".to_string(),
            ip: [80, 53, 53, 53],
        }
    }

    fn create_machine(&self, new_machine: Machine) {
        println!(">>> create machine");
    }


    fn delete_machine(&self, id: Uuid) {
        println!(">>> delete {{{}}} machine", id.as_u128());
    }

    fn update_machine(&self, id: Uuid, upd_machine: Machine) {
        println!(">>> update {{{}}} machine", id.as_u128());
    }
}