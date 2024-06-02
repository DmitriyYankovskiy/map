pub mod machine;
pub mod project;

use uuid::Uuid;
use project::Project;
use machine::Machine;

pub trait Data {
    fn get_project_by_id(&self, id: Uuid) -> Result<Project, String>;
    fn create_project(&self, new_project: &Project) -> Uuid;
    fn del_project_by_id(&self, id: Uuid) -> Result<(), String>;
    fn upd_project_by_id(&self, id: Uuid, upd_project: &Project) -> Result<(), String>;

    fn get_machine_by_id(&self, id: Uuid) -> Result<Machine, String>;
    fn create_machine(&self, new_machine: &Machine) -> Uuid;
    fn del_machine_by_id(&self, id: Uuid) -> Result<(), String>;
    fn upd_machine_by_id(&self, id: Uuid, new_machine: &Machine) -> Result<(), String>;
}


#[derive(Clone)]
pub struct Postgres {
}


impl Data for Postgres {
    fn get_project_by_id(&self, id: Uuid) -> Result<Project, String> {
        Ok(Project {
            name: "a".to_string(),
            port: 6666,
            path: "/pppppp".to_string(),
            descr: "description".to_string(),
        })
    }

    fn create_project(&self, new_project: &Project) -> Uuid {
        println!(">>> create project");
        Uuid::from_u128(0xa1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8u128)
    }


    fn del_project_by_id(&self, id: Uuid) -> Result<(), String> {
        println!(">>> del {{{}}} project", id.as_u128());
        Ok(())
    }

    fn upd_project_by_id(&self, id: Uuid, upd_project: &Project) -> Result<(), String>{
        println!(">>> upd {{{}}} project", id.as_u128());
        Ok(())
    }


    fn get_machine_by_id(&self, id: Uuid) -> Result<Machine, String> {
        Ok(Machine {
            name: "a".to_string(),
            ip: [80, 53, 53, 53],
        })
    }

    fn create_machine(&self, new_machine: &Machine) -> Uuid {
        println!(">>> create machine");
        Uuid::from_u128(0xa1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8u128)
    }


    fn del_machine_by_id(&self, id: Uuid) -> Result<(), String>{
        println!(">>> del {{{}}} machine", id.as_u128());
        Ok(())
    }

    fn upd_machine_by_id(&self, id: Uuid, upd_machine: &Machine) -> Result<(), String> {
        println!(">>> upd {{{}}} machine", id.as_u128());
        Ok(())
    }
}