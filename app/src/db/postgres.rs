use super::{Machine, Unit, Status, Uuid, Data};

#[derive(Clone)]
pub struct Postgres {
}

pub fn connect() -> Postgres {
    Postgres {
    }
}

impl Data for Postgres {
    fn get_unit_by_id(&self, id: Uuid) -> Result<Unit, String> {
        Ok(Unit {
            name: "a".to_string(),
            port: 6666,
            path: "/pppppp".to_string(),
            descr: "description".to_string(),
            status: Status::Online,
            machine_id: 12212,
        })
    }

    fn create_unit(&self, new_unit: &Unit) -> Uuid {
        println!(">>> create unit");
        Uuid::from_u128(0xa1a2a3a4b1b2c1c2d1d2d3d4d5d6d7d8u128)
    }


    fn del_unit_by_id(&self, id: Uuid) -> Result<(), String> {
        println!(">>> del {{{}}} unit", id.as_u128());
        Ok(())
    }

    fn upd_unit_by_id(&self, id: Uuid, upd_unit: &Unit) -> Result<(), String>{
        println!(">>> upd {{{}}} unit", id.as_u128());
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