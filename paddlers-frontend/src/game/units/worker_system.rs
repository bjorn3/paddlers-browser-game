use specs::prelude::*;
use crate::game::{
    Now,
    movement::Moving,
    units::workers::Worker,
    town::Town,
};
use paddlers_shared_lib::models::*;
use quicksilver::geom::about_equal;

pub struct WorkerSystem;

impl<'a> System<'a> for WorkerSystem {
    type SystemData = (
        WriteStorage<'a, Worker>,
        WriteStorage<'a, Moving>,
        Read<'a, Town>,
        Read<'a, Now>,
     );

    fn run(&mut self, (mut workers, mut velocities, town, now): Self::SystemData) {
        for (mut worker, mut mov) in (&mut workers, &mut velocities).join() {
            if let Some(task) = worker.poll(now.0) {
                match task.task_type {
                    TaskType::Walk => {
                        let position_now = mov.position(task.start_time);
                        let position_after = town.tile_area(task.position).pos;
                        if about_equal(position_now.x, position_after.x)
                        && about_equal(position_now.y, position_after.y)  {
                            continue;
                        }
                        let dir = position_after - position_now;
                        mov.start_ts = task.start_time;
                        mov.start_pos = position_now;
                        mov.momentum = dir.normalize() * mov.max_speed;
                    },
                    TaskType::Idle => {
                        mov.start_pos = mov.position(task.start_time);
                        mov.start_ts = task.start_time;
                        mov.momentum = (0.0,0.0).into();
                    }
                    _ => {debug_assert!(false, "Unexpected task")},
                }
            }
        }
    }
}

