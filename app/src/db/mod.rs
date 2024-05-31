mod machine;
mod project;

use uuid::Uuid;
use project::Project;
use machine::Machine;

#[derive(Clone)]
pub struct Data {

}

impl Data {
    pub fn get_project_by_id(&self, id: Uuid) -> Project {
        Project {
            name: "a".to_string(),
            port: 6666,
            path: "/pppppp".to_string(),
            descr: "description".to_string(),
        }
    }

    pub fn create_project(&self, new_project: Project) {
        println!(">>> create project");
    }


    pub fn delete_project(&self, id: Uuid) {
        println!(">>> delete {{{}}} project", id.as_u128());
    }

    pub fn update_project(&self, id: Uuid, upd_project: Project) {
        println!(">>> update {{{}}} project", id.as_u128());
    }


    pub fn get_machine_by_id(&self, id: Uuid) -> Machine {
        Machine {
            name: "a".to_string(),
            ip: [80, 53, 53, 53],
        }
    }

    pub fn create_machine(&self, new_project: Project) {
        println!(">>> create machine");
    }


    pub fn delete_machine(&self, id: Uuid) {
        println!(">>> delete {{{}}} machine", id.as_u128());
    }

    pub fn update_machine(&self, id: Uuid, upd_project: Project) {
        println!(">>> update {{{}}} machine", id.as_u128());
    }
}