use paddlers_shared_lib::prelude::*;
use crate::db::DB;
use rand::Rng;

use actix::prelude::*;
use crate::db::*;

pub struct AttackSpawner {
    dbpool: Pool,
}

impl Actor for AttackSpawner {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Context<Self>) {
       println!("Attack Spawner is alive");
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
       println!("Attack Spawner is stopped");
    }
}

pub (super) struct AttackTarget(pub VillageKey);
impl Message for AttackTarget {
    type Result = ();
}


impl Handler<AttackTarget> for AttackSpawner {
    type Result = ();

    fn handle(&mut self, msg: AttackTarget, _ctx: &mut Context<Self>) -> Self::Result {
        self.spawn_random_attack(msg.0);
    }
}


impl AttackSpawner {
    pub fn new(dbpool: Pool) -> Self {
        AttackSpawner {
            dbpool: dbpool,
        }
    }
    fn db(&self) -> DB {
       (&self.dbpool).into()
    }

    fn spawn_random_attack(&self, village: VillageKey) {
        let vid = village.num();
        let now = chrono::Utc::now().naive_utc();
        use std::ops::Add;
        let arrival = now.add(chrono::Duration::seconds(15));
        let new_attack = NewAttack {
            departure: now,
            arrival: arrival,
            origin_village_id: vid,
            destination_village_id: vid,
        };
        let attack = self.db().insert_attack(&new_attack);



        let mut rng = rand::thread_rng();
        let n = rng.gen_range(2,5);
        for _ in 0 .. n {
            let unit = NewHobo {
                color: Some(Self::gen_color(&mut rng)),
                hp: rng.gen_range(1, 10), 
                speed: 0.1,
                home: vid,
            };
            let u = self.db().insert_hobo(&unit);
            let atu = AttackToHobo {
                attack_id: attack.id,
                hobo_id: u.id
            };
            self.db().insert_attack_to_hobo(&atu);
        }
    }

    fn gen_color<R>(rng: &mut R) -> UnitColor 
    where R: Rng
    {
        match rng.gen_range(0,3) {
            0 => UnitColor::Yellow,
            1 => UnitColor::Camo,
            2 => UnitColor::White,
            _ => panic!("RNG bug?")
        }
    }

}