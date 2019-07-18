use actix::prelude::*;
use std::time::Duration;
use chrono::NaiveDateTime;
use crate::db::*;
use paddlers_shared_lib::sql::GameDB;

pub struct GameMaster {
    last_attack: NaiveDateTime,
    dbpool: Pool,
}

impl GameMaster {
    pub fn new(dbpool: Pool) -> Self {
        GameMaster {
            last_attack: NaiveDateTime::from_timestamp(0, 0),
            dbpool: dbpool,
        }
    }
}

// Provide Actor implementation for our actor
impl Actor for GameMaster {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
       println!("Game Master is alive");
       let db: DB = (&self.dbpool).into();
       db.init_resources();
       self.game_cycle(ctx);
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
       println!("Game Master is stopped");
    }
}

impl GameMaster {
    fn game_cycle(&mut self, ctx: &mut Context<Self>) {
        println!("Cycle");

        let db: DB = (&self.dbpool).into();
        check_attacks(&db);

        let now = chrono::Local::now().naive_local();
        if now - self.last_attack >= chrono::Duration::seconds(10) {
            self.last_attack = now;
            db.spawn_random_attack();
        }

        ctx.run_later(Duration::from_secs(1), Self::game_cycle);
    }
}


fn check_attacks(db: &DB) {
    let attacks = db.attacks(None);
    let now = chrono::Local::now().naive_local();
    for atk in attacks.iter() {
        if atk.arrival < now {
            db.maybe_attack_now(atk, now -  atk.arrival);
        }
    }
}