use quicksilver::geom::Vector;
use specs::prelude::*;
use crate::gui::{
    render::Renderable,
    z::Z_UNITS,
    sprites::SpriteIndex,
    utils::*,
};
use crate::game::{
    input::Clickable,
    movement::{Position, Velocity},
    fight::Health,
};

#[derive(Default, Component)]
#[storage(NullStorage)]
pub struct Attacker;

pub fn insert_duck(world: &mut World, pos: impl Into<Vector>, speed: impl Into<Vector>, hp: i64, ul: f32) -> Entity {
    let pos = pos.into();
    world.create_entity()
        .with(Position::new(pos, (0.6*ul,0.4*ul), Z_UNITS))
        .with(Velocity::new(pos, speed))
        .with(
            Renderable {
                kind: RenderVariant::ImgWithImgBackground(SpriteIndex::Duck, SpriteIndex::Water),
            }
        )
        .with(Clickable)
        .with(Attacker)
        .with(Health::new_full_health(hp))
        .build()
}

use crate::net::graphql::attacks_query::{AttacksQueryAttacksUnits,AttacksQueryAttacks};
impl AttacksQueryAttacks {
    pub fn create_entities(&self, world: &mut World, ul: f32, time_zero: f64) -> Vec<Entity> {
        let zero = chrono::NaiveDateTime::from_timestamp((time_zero / 1000.0) as i64, (time_zero % 1000.0) as u32 * 1000_000);
        let time_alive = zero - self.arrival();
        self.units
            .iter()
            .enumerate()
            .map(|(i, u)|{u.create_entity(world, &time_alive, i, ul)})
            .collect()
    }
}
impl AttacksQueryAttacksUnits {
    // TODO: For entities already well into the map, compute the attacks so far.
    fn create_entity(&self, world: &mut World, time_alive: &chrono::Duration, pos_rank: usize, ul: f32) -> Entity {
        let v = -self.speed as f32 / (super::CYCLE_SECS * 1000) as f32 * ul;
        let start_x = 1000.0 - 30.0;
        let y = 300.0;
        let x = start_x + time_alive.num_milliseconds() as f32 * v;
        let pos = Vector::new(x,y) + attacker_position_rank_offset(pos_rank);
        let hp = self.hp;
        insert_duck(world, pos, (v as f32,0.0), hp, ul)
    }
}

fn attacker_position_rank_offset(pr: usize) -> Vector {
    let y = if pr % 2 == 1 { -20 } else { 0 };
    let x = 15 * pr as i32;
    (x,y).into()
}