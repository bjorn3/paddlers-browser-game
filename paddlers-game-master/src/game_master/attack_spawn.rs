use paddlers_shared_lib::models::*;
use crate::db::DB;
use rand::Rng;

impl DB {
    pub fn spawn_random_attack(&self) {
        // println!("Spawning attack");
        let now = chrono::Utc::now().naive_utc();
        use std::ops::Add;
        let arrival = now.add(chrono::Duration::seconds(15));
        let new_attack = NewAttack {
            departure: now,
            arrival: arrival,
        };
        let attack = self.insert_attack(&new_attack);



        let mut rng = rand::thread_rng();
        let n = rng.gen_range(2,5);
        for _ in 0 .. n {
            let unit = NewUnit {
                unit_type: UnitType::Basic,
                x: 0,
                y: 0,
                color: Some(UnitColor::Yellow),
                hp: rng.gen_range(3, 6), 
                speed: 1.0,
                home: 2 // for now, 1 = only visible town, 2 = homeless attacker
            };
            let u = self.insert_unit(&unit);
            let atu = AttackToUnit {
                attack_id: attack.id,
                unit_id: u.id
            };
            self.insert_attack_to_unit(&atu);
        }
    }

}