use db_lib::models::*;
use db_lib::sql::GameDB;
use crate::{
    db::DB,
    shop::Price,
    StringErr
};

impl DB {
    pub fn collect_reward<'a, I>(&self, units: I) 
    where
        I: IntoIterator<Item = &'a Unit>,
    {
        use std::ops::Add;
        let feathers = units.into_iter().map(reward_feathers).fold(0, i64::add);
        self.add_resource(ResourceType::Feathers, feathers).expect("Adding feathers.");
    }

    pub fn init_resources(&self) {
        use db_lib::strum::IntoEnumIterator;
        for res in ResourceType::iter()
        {
            let entity = Resource {
                resource_type: res, 
                amount: 0,
            };
            match self.insert_resource(&entity) {
                Err(e) => println!("Couldn't insert resource. {} probably already exists. Error: {}", res, e),
                _ => {}
            }
        }
    }

    pub fn try_spend(&self, p: &Price) -> StringErr {
        self.can_afford(p)?;
        self.spend(p);
        Ok(())
    }

    pub fn spend(&self, p: &Price) {
        for (res, n) in p.0.iter() {
            self.add_resource(*res, -*n).expect("Unchecked spending resources");
        }
    }
    pub fn can_afford(&self, p: &Price) -> StringErr {
        for (res, n) in p.0.iter() {
            if self.resource(*res) < *n {
                return Err(format!("Not enough {}", res));
            }
        }
        Ok(())
    }

}
fn reward_feathers(unit: &Unit) -> i64 {
    let f = (1.0 + unit.hp as f32 * unit.speed / 4.0).log2().ceil() as i64;
    println!("{:#?} gives {} feathers", &unit, f);
    f
}