use std::sync::Arc;
use uuid::Uuid;

use crate::db::{self, project::Project, machine::Machine};

#[derive(Clone)]
pub struct App {
    pub data: Arc<dyn db::Data + Sync + Send>,
}

impl App {
    pub fn get_project_by_id(&self, id: Uuid) -> Result<Project, String> {
        self.data.get_project_by_id(id)
    }

    pub fn create_project(&self, new_project: &Project) -> Uuid{
        println!(">>> create project");
        self.data.create_project(new_project)
    }


    pub fn del_project_by_id(&self, id: Uuid) -> Result<(), String> {
        println!(">>> delete {{{}}} project", id.as_u128());
        self.data.del_project_by_id(id)
    }

    pub fn upd_project_by_id(&self, id: Uuid, upd_project: &Project) -> Result<(), String> {
        println!(">>> update {{{}}} project", id.as_u128());
        self.data.upd_project_by_id(id, upd_project)
    }


    pub fn get_machine_by_id(&self, id: Uuid) -> Result<Machine, String> {
        self.data.get_machine_by_id(id)
    }

    pub fn create_machine(&self, new_machine: &Machine) -> Uuid {
        println!(">>> create machine");
        self.data.create_machine(new_machine)
    }


    pub fn del_machine_by_id(&self, id: Uuid) -> Result<(), String> {
        println!(">>> delete {{{}}} machine", id.as_u128());
        self.data.del_machine_by_id(id)
    }

    pub fn upd_machine_by_id(&self, id: Uuid, upd_machine: &Machine) -> Result<(), String> {
        println!(">>> update {{{}}} machine", id.as_u128());
        self.data.upd_machine_by_id(id, upd_machine)
    }
}