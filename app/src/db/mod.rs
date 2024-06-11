pub mod models;
pub mod postgres;

use uuid::Uuid;
use models::{Unit, Machine, Status};

pub trait Data {
    fn get_unit_by_id(&self, id: Uuid) -> Result<Unit, String>;
    fn create_unit(&self, new_unit: &Unit) -> Uuid;
    fn del_unit_by_id(&self, id: Uuid) -> Result<(), String>;
    fn upd_unit_by_id(&self, id: Uuid, upd_unit: &Unit) -> Result<(), String>;

    fn get_machine_by_id(&self, id: Uuid) -> Result<Machine, String>;
    fn create_machine(&self, new_machine: &Machine) -> Uuid;
    fn del_machine_by_id(&self, id: Uuid) -> Result<(), String>;
    fn upd_machine_by_id(&self, id: Uuid, new_machine: &Machine) -> Result<(), String>;
}